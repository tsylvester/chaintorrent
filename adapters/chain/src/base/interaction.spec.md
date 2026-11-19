# `BaseChain` — interaction spec

Branch contract for the `base` concrete of the `chain` crate: the chain family's Base concrete over `alloy`, constructed from its settings over two providers — one to the chain's endpoint, one to the settlement endpoint named by the settings — and implementing `ISettlementAdapter`, `IEntitlementStateAdapter`, and `IChainAdapter`. Each branch states condition, decision, dependency call, and the exact return outcome. A method's return is its family union — `CurrentReferenceAtReturn`, `TierOfReturn`, `ReferenceAgeReturn`, `EvaluateAuthorizationReturn`, `EvaluateAuthorizationBatchReturn`, `BatchPageLimitReturn`, `EndpointReturn`, `ChainIdentifierReturn`, `EntitlementContractReturn`, `EstimateFeeReturn`, and `RecoverEnvelopeReturn` — and the concrete's own error reaches it as the `Base` flavor of the family's error union.

## `BaseChain<F: IChainForms>::try_new(params: BaseChainConstructorParams<'_>) -> BaseChainTryNewReturn<F>`

The settings are read as the borrow the params carry and held by the instance; the endpoint text is passed to the transport as the settings hold it and is parsed by the transport alone; the forms are a type parameter and carry no value.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| runtime refused | the current-thread builder refuses | the build's return | `Builder::new_current_thread().enable_all().build()` once | `Err(BaseChainTryNewErrorReturn::Runtime(error))`, the runtime's `io::Error` intact; a refusal no input reaches and no block proves |
| endpoint refused | the transport refuses the settings' endpoint text | the connect's return | `ProviderBuilder::default().connect(endpoint_text)` awaited under the runtime, then `.erased()` | `Err(BaseChainTryNewErrorReturn::Endpoint(error))`, the `TransportError` intact |
| settlement endpoint refused | the transport refuses the settings' settlement endpoint text | the connect's return | `ProviderBuilder::default().connect(settlement_endpoint_text)` awaited under the runtime, then `.erased()` | `Err(BaseChainTryNewErrorReturn::SettlementEndpoint(error))`, the `TransportError` intact |
| constructed | every call above succeeds | none | none | `Ok(BaseChain { .. })` holding the two `DynProvider`s, the runtime, and the settings |

## `BaseChain<F>::try_new_mocked(params: BaseChainMockedConstructorParams<'_>) -> BaseChainTryNewReturn<F>` — `#[cfg(any(test, feature = "mocks"))]`

As `try_new`, the two providers built by `ProviderBuilder::default().connect_mocked_client(asserter)` from the params' `endpoint_transport` and `settlement_transport` `Asserter`s, then `.erased()`.

## `IChainAdapter::DECLARATION`

`BASE_CHAIN_DECLARATION`.

## `current_reference_at(params: CurrentReferenceAtParams, payload: CurrentReferenceAtPayload) -> CurrentReferenceAtReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| included | `params.tier` is `Included` | the head the tier's mapping names | `get_block_by_number(BlockNumberOrTag::Latest)` | `Ok` of a `SettlementReference` holding the block's number and hash |
| soft | `params.tier` is `Soft` | as `Included` | `get_block_by_number(BlockNumberOrTag::Safe)` | `Ok` of a reference holding the block's number and hash |
| hard | `params.tier` is `Hard` | as `Included` | `get_block_by_number(BlockNumberOrTag::Finalized)` | `Ok` of a reference holding the block's number and hash |
| settled | `params.tier` is `Settled` | the anchor's L2 block height, then the block at that height | `IAnchorStateRegistry.getAnchorRoot` on the settlement provider at the anchor state registry the settings name, decoded to the L2 block height; then `get_block_by_number(BlockNumberOrTag::Number(height))` on the endpoint | `Ok` of a reference holding that block's number and hash |
| anchor undecodable | `Settled` and the `getAnchorRoot` return does not decode | the binding's decode | none further | `Err(CurrentReferenceAtErrorReturn::Base(BaseCurrentReferenceAtErrorReturn::AnchorDecode))` |
| not attained | the block the tier resolves to is absent | the block read's return | none further | `Err(CurrentReferenceAtErrorReturn::NotAttained { tier })` |
| transport refused | any dependency call fails | the call's return | none further | `Err(CurrentReferenceAtErrorReturn::Base(BaseCurrentReferenceAtErrorReturn::Transport(error)))`, the vendor error intact |

## `tier_of(params: TierOfParams, payload: TierOfPayload<'_>) -> TierOfReturn`

Decision in order; `payload.reference` is the reference under test.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| not included | the chain's latest number is below the reference's ordinal, or no block exists at the ordinal | the ordinal against the chain | `get_block_by_number` of the latest block and of `Number(reference.ordinal)` | `Err(TierOfErrorReturn::NotIncluded { reference })` |
| reorganized | the block at the ordinal's hash differs from the reference's identity | the recorded identity against the chain's | none further | `Err(TierOfErrorReturn::Reorganized { reference })` |
| tier | otherwise | the highest of `Settled`, `Hard`, `Soft`, and `Included`, in that order, whose current reference's ordinal is at or after the reference's ordinal | each current reference read as `current_reference_at` reads it; a tier the chain has not attained is skipped | `Ok(TierOfSuccessReturn { tier })` |
| transport refused | any dependency call fails | the call's return | none further | `Err(TierOfErrorReturn::Base(BaseTierOfErrorReturn::Transport(error)))`, or `Base(AnchorDecode)` where the settled read's anchor does not decode, each vendor error intact |

## `reference_age(params: ReferenceAgeParams, payload: ReferenceAgePayload<'_>) -> ReferenceAgeReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| not included | the reference is not included — its ordinal past the latest number or its block absent | the ordinal against the chain | `get_block_by_number` of the latest block and of the reference's ordinal | `Err(ReferenceAgeErrorReturn::NotIncluded { reference })` |
| age | otherwise | the latest block's timestamp less the reference block's, saturating | the two block reads | `Ok(ReferenceAgeSuccessReturn { age })`, `seconds` the latest block's timestamp less the reference block's; a reference block timestamped after the latest yields zero by saturating subtraction |
| transport refused | any dependency call fails | the call's return | none further | `Err(ReferenceAgeErrorReturn::Base(BaseReferenceAgeErrorReturn::Transport(error)))`, the vendor error intact |

## `evaluate_authorization(params: EvaluateAuthorizationParams, payload: EvaluateAuthorizationPayload<'_, Self::Forms>) -> EvaluateAuthorizationReturn`

Every conversion precedes the call: the two `F::Identity` fields — `entitlement_contract` and `identity` — by `to_field` to their twenty bytes; `entitlement` by `to_field` to its 32 bytes; `interval` by `to_field` to its 64-bit value; `asset`, `deployment`, and `handshake_key` by `as_bytes`; the scheme by `identifier`; the reference by its ordinal.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| not canonical | the endpoint reports the block the reference records as not canonical | the call's refusal | `evaluateAuthorization` on the registry the settings name, read through `hash_canonical` at the block whose hash the context's `reference` records | `Err(EvaluateAuthorizationErrorReturn::Reorganized { reference })` |
| transport refused | the call's transport fails | the call's return | as above | `Err(EvaluateAuthorizationErrorReturn::Base(BaseEvaluateAuthorizationErrorReturn::Transport(error)))`, the vendor error intact |
| undecodable | the return does not decode through the binding | the binding's decode | as above | `Err(EvaluateAuthorizationErrorReturn::Base(BaseEvaluateAuthorizationErrorReturn::Decode))` |
| authorized / pending / denied | the returned `uint8` is `0`, `1`, or `2` | the value | as above | `Ok` of `Authorized`, `PendingSettlement`, or `Denied` |
| unknown state | the returned `uint8` is outside `0`, `1`, `2` | the value | as above | `Err(EvaluateAuthorizationErrorReturn::Base(BaseEvaluateAuthorizationErrorReturn::UnknownAuthorizationState { value }))` |

## `evaluate_authorization_batch(params: EvaluateAuthorizationBatchParams, payload: EvaluateAuthorizationBatchPayload<'_, Self::Forms>) -> EvaluateAuthorizationBatchReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| page too large | `payload.contexts.len()` exceeds the settings' `batch_page_limit` | the count against the settings' limit, before any call | none | `Err(EvaluateAuthorizationBatchErrorReturn::PageTooLarge { limit, supplied })` |
| evaluated | otherwise | each context converted as `evaluate_authorization` converts it; the batch never broadens a context — a context's own fields are the only fields its result is read for | one `evaluateAuthorizationBatch` call with the contexts in order and `params.page.index`, read at the first context's reference through `hash_canonical` as the individual call is | `Ok` holding one result per context in the order given and `next` the page the registry returned when it is nonzero |
| not canonical | the block the first context's reference records is not canonical | the call's refusal | as above | `Err(EvaluateAuthorizationBatchErrorReturn::Reorganized { reference })` |
| result count differs | the registry returns a number of results other than the number of contexts | the two counts | as above | `Err(EvaluateAuthorizationBatchErrorReturn::Base(BaseEvaluateAuthorizationBatchErrorReturn::ResultCount { expected, actual }))` |
| transport refused / undecodable / unknown state | as the individual call | as the individual call | as above | the `Base` flavor carrying `Transport(error)`, `Decode`, or `UnknownAuthorizationState { value }` intact |

## `endpoint`, `chain_identifier`, `entitlement_contract`, and `batch_page_limit`

Each one branch: the settings' value. `endpoint` and `batch_page_limit` cannot fail; `chain_identifier` is rebuilt through the forms' `from_field` over the settings' 32 bytes and `entitlement_contract` through `from_field` over the settings' twenty bytes, a refusal carried intact as `Err(ChainIdentifierErrorReturn::Base(BaseChainIdentifierErrorReturn::Form(error)))` and `Err(EntitlementContractErrorReturn::Base(BaseEntitlementContractErrorReturn::Form(error)))`. No input reaches those two refusals, since each form's kind is the kind the settings' bytes are admitted into, so they are proven by no block.

## `estimate_fee(params: EstimateFeeParams, payload: EstimateFeePayload<'_, Self::Forms>) -> EstimateFeeReturn`

The request's `to` converted to its twenty bytes precedes the calls; nothing is signed, sent, or settled.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| estimated | the calls succeed and the fee fits | the oracle's return against `u128::MAX` | `estimate_gas` over a `TransactionRequest` holding the destination and the input; then `IGasPriceOracle.getL1Fee` on the gas price oracle the settings name over `TxEip1559 { chain_id, nonce: 0, gas_limit, max_fee_per_gas: 0, max_priority_fee_per_gas: 0, to, value: 0, input, access_list: empty }.encoded_for_signing()` | `Ok` of a `FeeDescription` whose entries carry, in the order `DECLARATION.fee_entries` states, each entry's name and unit and its quantity — the execution gas converted through `u128::from` and the L1 data fee |
| fee out of range | the oracle's fee does not fit `u128` | the fee against `u128::MAX` | as above | `Err(EstimateFeeErrorReturn::Base(BaseEstimateFeeErrorReturn::FeeOutOfRange { fee }))`, the fee as 32 big-endian bytes |
| transport refused / undecodable | a call fails or a return does not decode | the call's return | as above | `Err(EstimateFeeErrorReturn::Base(BaseEstimateFeeErrorReturn::Transport(error)))` or `Base(Decode)`, each intact |

## `recover_envelope(params: RecoverEnvelopeParams, payload: RecoverEnvelopePayload<'_, Self::Forms>) -> RecoverEnvelopeReturn`

The `entitlement` converted to its 32 bytes and the `interval` to its 64-bit value precede the calls; the envelope bytes are returned undecoded.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| not found | no log's interval equals the request's — `Granted` recording interval zero | each decoded event's interval against the request's | `get_logs` once per settlement event whose entitlement is the request's — `Minted`, `Delivered`, and `Replaced` on the first indexed position and `Granted` on its second — from the earliest block | `Err(RecoverEnvelopeErrorReturn::NotFound)` |
| recovered | a log of the request's interval exists and its calldata agrees | the event's envelope against the calldata's | `get_transaction_by_hash` of the log's transaction, its input decoded through the bound call of that settlement | `Ok(RecoverEnvelopeSuccessReturn { recovered })` — the event's envelope and the transaction hash |
| undecodable | a log or an input does not decode through its binding | the binding's decode | as above | `Err(RecoverEnvelopeErrorReturn::Base(BaseRecoverEnvelopeErrorReturn::Decode))` |
| calldata disagrees | the calldata's envelope differs from the event's | the two byte strings | as above | `Err(RecoverEnvelopeErrorReturn::Base(BaseRecoverEnvelopeErrorReturn::CalldataDisagrees))` |
| transport refused | a call's transport fails | the call's return | as above | `Err(RecoverEnvelopeErrorReturn::Base(BaseRecoverEnvelopeErrorReturn::Transport(error)))`, the vendor error intact |

## Ordering and invariants

- Every conversion precedes its call; a failed call precedes no later call; the same inputs read against the same chain state yield the same outcome.
- No method signs, sends, or writes; `estimate_fee` estimates without signing, sending, or settling anything.
- `current_reference_at`'s block tags are the tier's own; `tier_of` reads each tier's head as `current_reference_at` reads it and skips a tier the chain has not attained.
- The two providers are distinct dependencies: the anchor state registry is read on the settlement endpoint, every other read on the endpoint.

## Integration: callee dispositions

Carried, restated in this node's terms:

- `chain/create_chain_forms` entries `chain_forms_identity_survives_its_field_round_trip`, `chain_forms_entitlement_survives_its_field_round_trip`, `chain_forms_interval_survives_its_field_round_trip`, and `chain_forms_chain_identifier_survives_its_field_round_trip`: condition an identity, an entitlement, an interval, and a chain identifier form is handed to `evaluate_authorization` and the same forms are read back through `entitlement_contract` and `chain_identifier`; outcome the value that reaches the call data is the field the form yields and the form rebuilt equals the form that yielded it; route `create_chain_forms` over the forms concrete, `BaseChain::evaluate_authorization`, `BaseChain::entitlement_contract`, and `BaseChain::chain_identifier`, ending at the node; outer-edge collaborators the providers.
- `domain/handshake_key_commitment` entry `an_admitted_commitment_reads_back_its_bytes_and_its_scheme`: condition a commitment admitted with nonzero bytes and a scheme differing from the builder's is the context's handshake key; outcome the context call data carries the commitment's bytes and the scheme's identifier unchanged; route `HandshakeKeyCommitment::try_new`, `as_bytes`, `scheme`, and `BaseChain::evaluate_authorization`; outer-edge collaborator the provider.

Neither passing through a function of this node nor ending at one of its tests:

- the `chain/evm_forms` entries `evm_identity_form_survives_its_field_round_trip`, `evm_entitlement_form_survives_its_field_round_trip`, and `evm_interval_form_survives_its_field_round_trip` and the entries of its other forms, which end at that node's tests, this node reaching the forms only through `create_chain_forms`;
- the `chain/create_chain_forms` entries `chain_forms_refuse_an_encoder_declaring_another_encoding_identifier`, `chain_forms_handed_to_the_consumer_declare_the_required_identifier`, `chain_forms_key_identifier_survives_its_field_round_trip`, and `chain_forms_signed_intent_survives_its_field_round_trip`, this node calling no admission, reading no declaration of the forms, and using no key identifier or signed intent;
- the `hashing/blake3_bao` entries `commit_under_a_key_leaves_the_borrowed_key_intact`, `committed_outboard_verifies_its_content`, `extracted_paths_verify_every_chunk`, `a_challenge_response_verifies_against_the_root`, and `keyed_paths_verify_only_under_their_key`, this node reading `Root::as_bytes` and calling no commitment operation;
- the `domain/signature_scheme_identifier` and `domain/deployment_identity` entries, this node reading their accessors only;
- the `chain/base/settings` entry `the_declared_settings_are_admitted_through_the_real_constructors`, this node reading the settings type and the declaration constant and calling no admission.

## Integration: own entries

- `each_tier_reads_its_own_head_and_orders_the_references`: condition `current_reference_at` is called for each of the four tiers against an endpoint whose latest, safe, and finalized blocks and whose anchor state hold four distinct heights; outcome each reference's ordinal is its tier's height and the references compare in the order of the tiers, `Included` greatest; variation the four tiers, so a mapping that reads the same head for two tiers or reverses two fails; edge that must survive the composition: the block tag the transport received for each tier is the tier's.
- `a_context_reaches_the_registry_with_every_field`: condition `evaluate_authorization` is called with a context whose every field differs from the builders' defaults; outcome the `evaluateAuthorization` call the provider received decodes to a context equal field by field to the one handed; variation each of the nine members set to a distinct value, so a call that drops, defaults, or reorders a member fails; edge that must survive the composition: the context is the only input of the call.
- `batch_evaluation_agrees_with_individual_evaluation`: condition `evaluate_authorization_batch` and `evaluate_authorization` are called over the same contexts against a registry answering each context by its asset; outcome the batch's result at each position equals the individual result of that position's context; variation three contexts of different assets, identities, and references whose results are `Authorized`, `PendingSettlement`, and `Denied`, so a batch that reorders or broadens a result fails; edge that must survive the composition: the per-context binding survives pagination, the next page returned unchanged.
- `a_reorganized_reference_is_refused_whatever_its_number`: condition `tier_of` and `evaluate_authorization` are called with a reference whose number the chain holds under another hash; outcome `Err(Reorganized { reference })` from each; variation the reference at the latest, safe, and finalized heads, so a refusal that reads only the number fails; edge that must survive the composition: the identity the reference recorded decides.
- `a_delivered_envelope_is_recovered_from_the_settlement_that_created_the_interval`: condition `recover_envelope` is called for an interval created by a mint, a delivery, a grant, and a replacement in turn against a history holding all four; outcome each returns the envelope of its own settlement and its transaction hash; variation the four events and a history holding an earlier interval of the same entitlement, so a recovery that returns the first or the last event fails; edge that must survive the composition: the calldata confirms the event.
- `an_estimated_fee_carries_its_execution_gas_and_its_l1_data_fee_in_the_declared_order`: condition `estimate_fee` is called against an endpoint answering a gas estimate and an oracle answering an L1 fee; outcome the description's entries are the declaration's names and units in its order with the two values; variation an oracle fee above `u128::MAX`, outcome the range refusal; edge that must survive the composition: nothing is signed or sent.

## Integration: private surface

The chain of real functions `create_chain_forms` over the forms concrete resolved through the encoding factory, `BaseChain::try_new_mocked`, and each method of `BaseChain`, and the sol bindings running for real; the outer-edge collaborators mocked are the two providers' transports, through `Asserter`; the observable results are the calls the transports received and each method's return; the entries proven are the own entries above and the carried callee entries.

## Integration: public surface

None. This node's boundary is not a public entry, `chain/factory` being the family's public entry, so it carries no public integration test.
