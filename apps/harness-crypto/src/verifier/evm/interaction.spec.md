# `EvmVerifier` — interaction spec

Branch contract for the `evm` module of the `verifier` module of `apps/harness-crypto`: the EVM concrete of the harness's verifier family, one adapter over the borrowed pairing and delivery proof adapters, which reads a statement and the proof the delivery proof family produced for it, submits them to the deployed delivery verifier as the calldata the suite's ABI description fixes, and returns the chain's verdict with the cost of an accepted delivery as L2 execution gas and L1 data fee. Each branch states condition, decision, dependency call, and the exact return outcome.

## `EvmVerifierEndpoint::try_new(params: EvmVerifierEndpointConstructorParams) -> EvmVerifierEndpointTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| refused | `Url::parse(&params.text)` returns `Err(error)` | the URL parse | none | `Err(EvmVerifierEndpointTryNewErrorReturn::Unparsable { error })`, `error` unchanged |
| admitted | `Url::parse(&params.text)` returns `Ok(url)` | none | none | `Ok(EvmVerifierEndpoint { url })` |

## `EvmVerifier::try_new(params: EvmVerifierConstructorParams<'a, P, D>) -> EvmVerifierTryNewReturn<'a, P, D>`

The addresses are admitted, then the provider and runtime are built, the first refusal returning.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| verifier address refused | the identity form refuses `params.settings.verifier_address` | `from_field` | `<D::Forms as IChainForms>::Identity::from_field(FromFieldParams, params.settings.verifier_address)` | `Err(EvmVerifierTryNewErrorReturn::Identity { field: EvmVerifierField::VerifierAddress, error })`, the form's refusal unchanged |
| verifier address of another kind | `from_field` admits and `to_field(ToFieldParams, &form)` unpacked irrefutably yields a value other than `CanonicalFieldValue::FixedBytes20(bytes)` | an exhaustive `match` on the field value | `to_field` | `Err(EvmVerifierTryNewErrorReturn::IdentityKind { field: EvmVerifierField::VerifierAddress })`, a branch no form the repository declares takes and which has no unit test |
| oracle address refused | the identity form refuses `params.settings.gas_price_oracle` | the same two decisions under `EvmVerifierField::GasPriceOracle` | `from_field`, then `to_field` | `Err(EvmVerifierTryNewErrorReturn::Identity { field: EvmVerifierField::GasPriceOracle, error })`, then `IdentityKind { field: EvmVerifierField::GasPriceOracle }` on the unreachable kind branch |
| runtime refused | both addresses admitted and the runtime build returns `Err(error)` | the build's result | `Builder::new_current_thread().enable_all().build()` | `Err(EvmVerifierTryNewErrorReturn::Runtime(error))`, `error` unchanged, a branch no input reaches and which has no unit test |
| admitted | both addresses admitted | the provider built over the endpoint's URL, then the runtime | `ProviderBuilder::default().connect_http(url.clone()).erased()` over `params.settings.endpoint`'s URL | `Ok(EvmVerifier { pairing, delivery_proof, verifier, oracle, provider, runtime })`, each address `Address::from(bytes)` |

## `EvmVerifier::try_new_mocked(params: EvmVerifierConstructorParams<'a, P, D>, asserter: Asserter) -> EvmVerifierTryNewReturn<'a, P, D>`

Under `#[cfg(any(test, feature = "mocks"))]`. The same decisions in the same order with the provider `ProviderBuilder::default().connect_mocked_client(asserter).erased()` in place of the HTTP provider and the endpoint unread; the one place a provider other than the endpoint's is built, so a block drives the real concrete over a scripted chain.

## `IVerifierAdapter for EvmVerifier<'_, P, D>`

`const DECLARATION: VerifierDeclaration` naming `EVM_VERIFIER_DECLARATION`, which holds `verifier_forms` `&[VerifierGroupArithmetic::FirstGroupOnly, VerifierGroupArithmetic::BothGroups]`, `statement_versions` `&[DELIVERY_STATEMENT_VERSION_ONE]`, `adapter_version` `1`, and `interface_version` `VERIFIER_INTERFACE_VERSION`; `type DeliveryProof = D`.

## `verify_on_chain(&self, params: VerifyOnChainParams, payload: VerifyPayload<'_, D::Forms, P::G1, P::G2, D::Proof>) -> VerifyOnChainReturn`

The trusted form. `F` below is `D::Forms`; `params` carries nothing and is not read.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| calldata refused | `self.calldata` returns `Err(error)` | the proof's components are read once, unpacked irrefutably | `self.delivery_proof.proof_components(ProofComponentsParams, ProofComponentsPayload { proof: payload.proof })`, then `self.calldata(&payload.statement, components)` | `Err`, the refusal returned unchanged |
| resolved | `self.calldata` returns `Ok(calldata)` | none | `self.read_chain(calldata)` | `self.read_chain`'s return is the function's return |

## `calldata(&self, statement: &AlgebraicStatement<'_, F, P::G1, P::G2>, components: DeliveryProofComponents<P::Scalar, P::G2>) -> Result<Vec<u8>, VerifyOnChainErrorReturn>`

Private. The decision is an exhaustive `match` on `(statement, components)`.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| mint, both groups | `AlgebraicStatement::Mint` with `DeliveryProofComponents::MintBothGroups { challenge, responses }` | none | `self.mint_calldata(statement, challenge, responses, vec![])` | the helper's return, `Err(error)` mapped `Err(VerifyOnChainErrorReturn::Evm(error))`, `error` unchanged |
| mint, first group only | `AlgebraicStatement::Mint` with `DeliveryProofComponents::MintFirstGroupOnly { challenge, responses, first_messages }` | none | `self.mint_calldata(statement, challenge, responses, vec![first_messages.hpub, first_messages.d1, first_messages.d2])`, the first messages flattened in the order `[hpub, d1, d2]` | as above |
| transfer, both groups | `AlgebraicStatement::Transfer` with `DeliveryProofComponents::TransferBothGroups { challenge, responses }` | none | `self.transfer_calldata(statement, challenge, responses, vec![])` | as above |
| transfer, first group only | `AlgebraicStatement::Transfer` with `DeliveryProofComponents::TransferFirstGroupOnly { challenge, responses, first_messages }` | none | `self.transfer_calldata(statement, challenge, responses, vec![first_messages.pk2, first_messages.d1, first_messages.d2])`, the first messages flattened in the order `[pk2, d1, d2]` | as above |
| relation mismatch | a mint statement with transfer components, or a transfer statement with mint components | the same `match`, before any encoding | none | `Err(VerifyOnChainErrorReturn::RelationMismatch { statement: DeliveryRelation::Mint, proof: DeliveryRelation::Transfer })`, or `RelationMismatch { statement: DeliveryRelation::Transfer, proof: DeliveryRelation::Mint }` |

## `delivery_context<Purpose>(&self, context: &DeliveryContext<F, Purpose>, purpose: u16) -> Result<IDeliveryVerifier::DeliveryContext, EvmVerifierErrorReturn>`

Private. Each form's `to_field(ToFieldParams, &form)` is unpacked irrefutably and collected into `CanonicalFields`, in the order chain identifier, entitlement contract, source entitlement, target entitlement, old interval, new interval, seller, buyer.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| reader refused | `CanonicalFieldReader::try_new` or any `read_*` refuses | the field kinds against `EVM_VERIFIER_CONTEXT_FIELD_KINDS` | `CanonicalFieldReader::try_new(fields, EVM_VERIFIER_CONTEXT_FIELD_KINDS)`, then `read_unsigned256` for the chain identifier, `read_fixed_bytes20` for the entitlement contract, `read_unsigned256` for the source and target entitlements, `read_unsigned64` for the old and new intervals, and `read_fixed_bytes20` for the seller and buyer, in that order | `Err(EvmVerifierErrorReturn::Fields(error))`, `error` unchanged, a branch no form the repository declares takes and which has no unit test |
| read | every read admits | none | none | `Ok(IDeliveryVerifier::DeliveryContext)` with `suiteIdentifier` `*context.suite_identifier.identifier()`, `suiteVersion` `context.suite_identifier.version()`, `assetRoot` `*context.asset_root.as_bytes()`, `parameterSet` `*context.parameter_set_identifier.as_bytes()`, `purpose` the `purpose` argument, the purpose's code, `expiry` `context.expiry`, each `Unsigned256` as `U256::from_be_bytes`, and each `FixedBytes20` as `FixedBytes::from` |

## `encode_scalar_word(&self, scalar: P::Scalar) -> Result<U256, EvmVerifierErrorReturn>`

Private.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| length refused | the encoded bytes' length is not thirty-two | the byte length | `self.pairing.encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar })`, the bytes read by `expose().as_ref()` | `Err(EvmVerifierErrorReturn::ScalarLength { actual })`, a branch no pairing the repository declares takes and which has no unit test |
| encoded | the length is thirty-two | none | as above | `Ok(U256::from_be_bytes)` over the thirty-two bytes |

## `mint_calldata(&self, statement: &AlgebraicMintStatement<F, P::G1, P::G2>, challenge: P::Scalar, responses: MintResponses<P::Scalar>, first_messages: Vec<P::G2>) -> Result<Vec<u8>, EvmVerifierErrorReturn>`

Private. The context is read by `delivery_context` with `statement.context.purpose.code()`, then each of `hpub`, `identity_element`, `buyer_keys.pk1`, `buyer_keys.pk2`, `envelope.c1`, `envelope.c2`, `envelope.d1`, and `envelope.d2` through `encode_g1` or `encode_g2` by the group of its type into `IDeliveryVerifier::MintStatement`'s member of that name, then `challenge`, `alpha`, `r`, `rho`, and `sigma` through `encode_scalar_word` into `IDeliveryVerifier::MintProof`, `secondGroupFirstMessages` the `encode_g2` bytes of `first_messages` concatenated in order; the first refusal returns, the context before the group elements before the scalars; outcome `Ok` of `IDeliveryVerifier::verifyMintCall { statement, proof }.abi_encode()`.

## `transfer_calldata(&self, statement: &AlgebraicTransferStatement<F, P::G1, P::G2>, challenge: P::Scalar, responses: TransferResponses<P::Scalar>, first_messages: Vec<P::G2>) -> Result<Vec<u8>, EvmVerifierErrorReturn>`

Private, as `mint_calldata` over `identity_element`, `seller_keys.pk1`, `seller_keys.pk2`, `buyer_keys.pk1`, `buyer_keys.pk2`, `old_envelope.c1` through `d2`, and `new_envelope.c1` through `d2`, the scalars `challenge`, `x`, `y`, `s`, `rho`, and `sigma`, and `IDeliveryVerifier::verifyTransferCall`.

## `unsigned_transaction(chain_id: u64, gas_limit: u64, verifier: Address, calldata: &[u8]) -> Vec<u8>`

Private and pure, an associated function of `EvmVerifier` taking no `self`: the EIP-1559 transaction `TxEip1559 { chain_id, nonce: 0, gas_limit, max_fee_per_gas: 0, max_priority_fee_per_gas: 0, to: TxKind::Call(verifier), value: U256::ZERO, access_list: AccessList::default(), input: calldata.to_vec().into() }` returned as its `encoded_for_signing()`, the byte `0x02` and the RLP list of the nine fields, the form the oracle's `getL1Fee` takes.

## `verifier_request(&self, calldata: &[u8]) -> TransactionRequest` and `fee_request(&self, unsigned: &[u8]) -> TransactionRequest`

Private. `verifier_request` is `TransactionRequest::default().with_to(self.verifier).with_input(calldata.to_vec())`; `fee_request` is `TransactionRequest::default().with_to(self.oracle).with_input(IGasPriceOracle::getL1FeeCall { _data: unsigned.to_vec().into() }.abi_encode())`.

## `read_chain(&self, calldata: Vec<u8>) -> VerifyOnChainReturn`

Private. Every read runs under `self.runtime.block_on` over `self.provider`, and every transport refusal is `Err(VerifyOnChainErrorReturn::Evm(EvmVerifierErrorReturn::Rpc(error)))`, `error` unchanged; the reads in the order below, the first refusal or verdict returning.

### Verifier code

`get_code_at(self.verifier)`; a refusal returns `Rpc`; an empty result returns `Err(VerifyOnChainErrorReturn::Evm(EvmVerifierErrorReturn::NothingDeployed { address }))`, `address` the verifier's twenty bytes.

### Verifying call

`call(self.verifier_request(&calldata))`; a refusal whose `as_error_resp()` is `Some(payload)` with `payload.as_revert_data()` `Some(data)` returns `Ok(VerifyOnChainSuccessReturn::Reverted { data: data.to_vec() })`, and any other refusal, a transport failure or a JSON-RPC error carrying no revert data, returns `Rpc`; a returned value is decoded by `bool::abi_decode` through `SolValue`, the one decode both functions' returns share, a refusal returning `Err(VerifyOnChainErrorReturn::Evm(EvmVerifierErrorReturn::ResultDecode(error)))`, `false` returning `Ok(VerifyOnChainSuccessReturn::Rejected)`, and `true` continuing.

### L2 execution gas

`estimate_gas(self.verifier_request(&calldata))`; a refusal returns `Rpc`; the value is `l2_execution_gas`.

### L1 data fee

`get_code_at(self.oracle)`; a refusal returns `Rpc`; an empty result returns `Ok(VerifyOnChainSuccessReturn::Accepted { cost: VerifyCost { l2_execution_gas, l1_data_fee: L1DataFee::NotApplicable } })`; otherwise `get_chain_id()`, a refusal returning `Rpc`, then `unsigned_transaction(chain_id, l2_execution_gas, self.verifier, &calldata)` handed to `call(self.fee_request(&unsigned))`, any refusal, a revert included, returning `Rpc`; a returned value that `U256::abi_decode` through `SolValue` refuses returns `Err(VerifyOnChainErrorReturn::Evm(EvmVerifierErrorReturn::ResultDecode(error)))`; a fee for which `u128::try_from` fails returns `Err(VerifyOnChainErrorReturn::Evm(EvmVerifierErrorReturn::L1FeeOutOfRange { fee }))`, `fee` the big-endian bytes of the `U256`; otherwise `Ok(VerifyOnChainSuccessReturn::Accepted { cost: VerifyCost { l2_execution_gas, l1_data_fee: L1DataFee::Charged { wei } } })`.

## Ordering and invariants

- The relation, the context, the group elements, the scalars, then the reads — the verifier's code, the verifying call, the gas estimate, the oracle's code, the chain identifier, and the oracle's fee; a refusal or verdict before the reads leaves the chain unread, and the response queue of a scripted chain untouched.
- The requests the reads send are `verifier_request` and `fee_request`, and the transaction the fee is read over is `unsigned_transaction`, each a private method a unit test calls, since a scripted chain records no request.

## Integration: own entries

Each entry is proven by the private integration block of the same name in `integration_test.rs`.

- `a_real_proofs_calldata_decodes_to_its_statement_and_proof_at_the_pairings_encoding_lengths`: condition `calldata` over the statement `build_algebraic_mint_statement` or `build_algebraic_transfer_statement` builds and the components of the proof the real delivery proof adapter's `prove_mint` or `prove_transfer` returns over the witnesses `build_prove_mint_payload` or `build_prove_transfer_payload` holds, read by `proof_components`, the calldata decoded by `verifyMintCall::abi_decode` or `verifyTransferCall::abi_decode`; outcome every `bytes` member that holds a first-group element is the length of the first group's precompile encoding of the case's curve, `64` on BN254 and `128` on BLS12-381, every member that holds a second-group element `128` on BN254 and `256` on BLS12-381, `secondGroupFirstMessages` three second-group encodings long, in the order `hpub`, `d1`, `d2` for a mint and `pk2`, `d1`, `d2` for a transfer, where the pairing declares `VerifierGroupArithmetic::FirstGroupOnly` and empty where it declares `BothGroups`, and every scalar member the thirty-two-byte word of the encoded scalar; variation every member of `PAIRING_CONCRETES`, both relations, so a first-group member holding a second-group encoding, a scalar narrowed, or first messages dropped or reordered fails; edge that must survive the composition: the lengths are the precompile encodings' the real pairing emits, stated by curve and never read from the pairing or the declaration, and every real encoder, forms, and proof component flows through the ABI the interface file fixes.
- `an_accepted_verdict_and_its_costs_reach_the_caller_for_a_real_proof`: condition `verify_on_chain` over the same statement and proof, on a provider scripted to answer the verifier's code non-empty, the verifying call `true`, the gas estimate a value other than every default, and the oracle's code first empty and then non-empty with a chain identifier and a fee; outcome `Ok(VerifyOnChainSuccessReturn::Accepted { cost })` with `l2_execution_gas` the scripted gas and `l1_data_fee` `NotApplicable` for the empty oracle code and `Charged { wei }` with the scripted fee for the non-empty; variation every member of `PAIRING_CONCRETES`, both relations, and both oracle states; edge that must survive the composition: the cost is read from the chain in the order the reads are made, so a swapped read or a fee read where the oracle has no code fails.
- `a_rejecting_chain_returns_rejected_for_a_real_proof`: condition as the accepted entry with the verifying call answering `false`; outcome `Ok(VerifyOnChainSuccessReturn::Rejected)` and the scripted answers after the verifying call unread; variation every member of `PAIRING_CONCRETES` and both relations; edge that must survive the composition: a rejected delivery is a verdict and reads no cost.
- `a_reverting_chain_returns_the_revert_data_for_a_real_proof`: condition as the accepted entry with the verifying call refused by a JSON-RPC error carrying the hex data `0x08c379a0`; outcome `Ok(VerifyOnChainSuccessReturn::Reverted { data })` with `data` `[0x08, 0xc3, 0x79, 0xa0]` and the answers after unread; variation every member of `PAIRING_CONCRETES` and both relations; edge that must survive the composition: the revert data is the chain's, carried whole, and a revert is a verdict, never an `Rpc` error.
- `a_real_proof_of_the_other_relation_is_refused_before_any_chain_read`: condition `verify_on_chain` over a real mint statement and the real proof `prove_transfer` returned, and over a real transfer statement and the proof `prove_mint` returned, the scripted provider holding one answer; outcome `Err(VerifyOnChainErrorReturn::RelationMismatch { statement, proof })` with the relations the case names and the scripted answer unread; variation every member of `PAIRING_CONCRETES` and both orders; edge that must survive the composition: the refusal is the family's flavor, decided from the real proof's components before the chain is read, and names no concrete.

## Integration: callee dispositions

- `chain/create_chain_forms` entries `chain_forms_identity_survives_its_field_round_trip`, `chain_forms_entitlement_survives_its_field_round_trip`, `chain_forms_interval_survives_its_field_round_trip`, and `chain_forms_chain_identifier_survives_its_field_round_trip`: carried, restated in this node's terms — the identity forms at the entitlement contract, the seller, and the buyer reach the calldata as twenty-byte members, the entitlement forms as 256-bit members, the interval forms as 64-bit members, and the chain-identifier form as a 256-bit member, each the field its form's `to_field` yields; route `create_pairing`, the arm's `try_new`, `consume_pairing`, `create_chain_forms`, `consume_chain_forms`, `create_delivery_proof`, `consume_delivery_proof`, `delivery_context`, and the form's `to_field`; the route reaches no outer-edge collaborator.
- `proof/factory` entry `a_proof_the_adapter_rebuilds_from_its_components_verifies`: carried, restated in this node's terms — the components `proof_components` reads from a real proof, the challenge, the responses in witness order, and the second-group first messages the adapter's form carries, reach the calldata in that order, a mint's `alpha`, `r`, `rho`, `sigma` and a transfer's `x`, `y`, `s`, `rho`, `sigma`, then the first messages; route `create_pairing`, `create_delivery_proof`, `prove_mint` or `prove_transfer`, `proof_components`, `mint_calldata` or `transfer_calldata`; the route reaches no outer-edge collaborator.
- `proof/factory` entries `a_mint_proof_the_adapter_returns_verifies` and `a_transfer_proof_the_adapter_returns_verifies`: carried, restated in this node's terms — the real proofs the own entries encode are proofs the reference verifies for the statement they are formed over, and the chain's verdict on them is the deployed contract's, observed by the harness's run and not by this node; route `create_pairing`, `create_delivery_proof`, `prove_mint` or `prove_transfer`, `proof_components`, then `verify_on_chain`; the outer-edge collaborator is the scripted provider, the chain.
- No `encoding/factory` entry and no other `proof/factory` or `chain/create_chain_forms` entry has a route that reaches a function this node authors or its tests run; no function of this node consumes an encoder.

## Integration: private surface

The chain of real functions `create_pairing`, the arm's concrete through its `try_new`, `create_hash_to_scalar`, the selected concrete's `try_new`, `create_encoding`, `create_chain_forms`, `create_delivery_proof`, the selected concrete's `try_new`, `prove_mint`, `prove_transfer`, `proof_components`, `create_random_source` for `RandomSourceKind::OperatingSystem`, `EvmVerifier::try_new_mocked`, `verify_on_chain`, `calldata`, `mint_calldata`, `transfer_calldata`, `delivery_context`, `encode_scalar_word`, `unsigned_transaction`, `verifier_request`, `fee_request`, and `read_chain`; the cases are every member of `PAIRING_CONCRETES` crossed with the selections `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declare `HashToScalarIdentifier::Keccak256V1`, `DECLARED_ENCODING_SELECTIONS`, `CHAIN_FORMS_CONCRETES`, and `DELIVERY_PROOF_CONCRETES`; the outer-edge collaborators are the chain, a provider scripted through `Asserter`, and the operating system's generator running real; the observable result is the `VerifyOnChainReturn`, the decoded calldata, and the scripted queue's remaining answers; the entries proven are every own entry above and the carried entries.
