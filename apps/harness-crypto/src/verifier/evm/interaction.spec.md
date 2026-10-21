# evm interactions

`EvmVerifier` is the verifier family's EVM concrete: `verify_on_chain` reads a statement and the proof the delivery proof family produced for it, maps them field by field into the calldata `IDeliveryVerifier` fixes, submits the calldata to the deployed delivery verifier over the injected provider, and returns the chain's verdict — `Accepted` carrying the cost as L2 execution gas and L1 data fee, `Rejected`, or `Reverted` carrying the revert data — or an error. It builds no statement and no proof, which the caller and the delivery proof family do, decides parity nowhere, which the caller reads from the verdicts, and reaches the chain through the provider alone, the family's only module that does. It names no curve, address, endpoint, chain identifier, or version: the pairing and delivery proof adapters arrive by injection, the addresses and endpoint in its constructor params.

## `EvmVerifierEndpoint::try_new`

`try_new(params: EvmVerifierEndpointConstructorParams) -> EvmVerifierEndpointTryNewReturn`, decided by `Url::parse(&params.text)`:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| refused | `Url::parse(&params.text)` returns `Err(error)` | the parse | `Url::parse`, once | `Err(EvmVerifierEndpointTryNewErrorReturn::Unparsable { error })`, the error unchanged |
| admitted | `Url::parse` returns `Ok(url)` | none | `Url::parse` | `Ok(EvmVerifierEndpoint { url })` |

## `EvmVerifier::try_new`

`try_new(params: EvmVerifierConstructorParams<'a, P, D>) -> EvmVerifierTryNewReturn<'a, P, D>`: the addresses are admitted, then the provider and runtime are built, the first refusal returning.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| verifier identity refused | the identity form refuses `params.settings.verifier_address` | the field's kind | `<D::Forms as IChainForms>::Identity::from_field(FromFieldParams, params.settings.verifier_address)` | `Err(EvmVerifierTryNewErrorReturn::Identity { field: EvmVerifierField::VerifierAddress, error })`, the form's refusal unchanged |
| verifier identity wrong kind | the admitted field is a `CanonicalFieldValue` other than `FixedBytes20(bytes)`, an exhaustive `match` | the kind | `to_field(ToFieldParams, &form)` unpacked irrefutably | `Err(EvmVerifierTryNewErrorReturn::IdentityKind { field: EvmVerifierField::VerifierAddress })`, a branch no form the repository declares takes and which has no unit test |
| oracle identity refused | the identity form refuses `params.settings.gas_price_oracle` | the field's kind | `Identity::from_field(FromFieldParams, params.settings.gas_price_oracle)` | `Err(EvmVerifierTryNewErrorReturn::Identity { field: EvmVerifierField::GasPriceOracle, error })`, the form's refusal unchanged |
| oracle identity wrong kind | the admitted field is a `CanonicalFieldValue` other than `FixedBytes20(bytes)`, an exhaustive `match` | the kind | `to_field(ToFieldParams, &form)` unpacked irrefutably | `Err(EvmVerifierTryNewErrorReturn::IdentityKind { field: EvmVerifierField::GasPriceOracle })`, a branch no form the repository declares takes and which has no unit test |
| runtime refused | `Builder::new_current_thread().enable_all().build()` returns `Err(error)` | the build | `Builder::new_current_thread().enable_all().build()`, after `ProviderBuilder::default().connect_http(url.clone()).erased()` over the endpoint's URL | `Err(EvmVerifierTryNewErrorReturn::Runtime(error))`, the error unchanged, a branch no input reaches and which has no unit test |
| admitted | both addresses admitted | the constructor | the provider call above | `Ok(EvmVerifier { pairing: params.pairing, delivery_proof: params.delivery_proof, verifier, oracle, provider, runtime })`, each address `Address::from(bytes)` |

The verifier's address is checked before the oracle's.

## `EvmVerifier::try_new_mocked`

`try_new_mocked(params: EvmVerifierConstructorParams<'a, P, D>, asserter: Asserter) -> EvmVerifierTryNewReturn<'a, P, D>`, under `#[cfg(any(test, feature = "mocks"))]`: the same decisions in the same order with the provider `ProviderBuilder::default().connect_mocked_client(asserter).erased()` in place of the HTTP provider and the endpoint unread. The one place a provider other than the endpoint's is built, so a block drives the real concrete over a scripted chain.

## `IVerifierAdapter for EvmVerifier<'_, P, D>`

`const DECLARATION: VerifierDeclaration` naming `EVM_VERIFIER_DECLARATION`, which holds `verifier_forms` `&[VerifierGroupArithmetic::FirstGroupOnly, VerifierGroupArithmetic::BothGroups]`, `statement_versions` `&[DELIVERY_STATEMENT_VERSION_ONE]`, `adapter_version` `1`, and `interface_version` `VERIFIER_INTERFACE_VERSION`; `type DeliveryProof = D`.

## `verify_on_chain`

`verify_on_chain(&self, params: VerifyOnChainParams, payload: VerifyAdapterPayload<'_, D>) -> VerifyOnChainReturn`, the trusted form; `F` below is `D::Forms`; `params` carries nothing and is not read.

| Step | Dependency call | A refusal |
|---|---|---|
| components | `self.delivery_proof.proof_components(ProofComponentsParams, ProofComponentsPayload { proof: payload.proof })`, once, unpacked irrefutably | none — the error arm is `Infallible` |
| calldata | `self.calldata(&payload.statement, components)` | returned unchanged |
| reads | `self.read_chain(calldata)` | the function's return |

## `calldata`

`calldata(&self, statement: &AlgebraicStatement<'_, F, P::G1, P::G2>, components: DeliveryProofComponents<P::Scalar, P::G2>) -> Result<Vec<u8>, VerifyOnChainErrorReturn>`, private. Decision an exhaustive `match` on `(statement, components)`:

| Branch | Condition | Decision | Outcome |
|---|---|---|---|
| mint, both groups | `AlgebraicStatement::Mint` with `DeliveryProofComponents::MintBothGroups { challenge, responses }` | the variant pair | `self.mint_calldata(statement, challenge, responses, Vec::new())` |
| mint, first group only | `AlgebraicStatement::Mint` with `DeliveryProofComponents::MintFirstGroupOnly { challenge, responses, first_messages }` | the variant pair | `self.mint_calldata(statement, challenge, responses, first_messages flattened in the order [hpub, d1, d2])` |
| transfer, both groups | `AlgebraicStatement::Transfer` with `DeliveryProofComponents::TransferBothGroups { challenge, responses }` | the variant pair | `self.transfer_calldata(statement, challenge, responses, Vec::new())` |
| transfer, first group only | `AlgebraicStatement::Transfer` with `DeliveryProofComponents::TransferFirstGroupOnly { challenge, responses, first_messages }` | the variant pair | `self.transfer_calldata(statement, challenge, responses, first_messages flattened in the order [pk2, d1, d2])` |
| mint statement, transfer components | `AlgebraicStatement::Mint` with `DeliveryProofComponents::TransferBothGroups` or `TransferFirstGroupOnly` | the variant pair, decided before any encoding | `Err(VerifyOnChainErrorReturn::RelationMismatch { statement: DeliveryRelation::Mint, proof: DeliveryRelation::Transfer })` |
| transfer statement, mint components | `AlgebraicStatement::Transfer` with `DeliveryProofComponents::MintBothGroups` or `MintFirstGroupOnly` | the variant pair, decided before any encoding | `Err(VerifyOnChainErrorReturn::RelationMismatch { statement: DeliveryRelation::Transfer, proof: DeliveryRelation::Mint })` |

A helper's `Err(error)` returns `Err(VerifyOnChainErrorReturn::Evm(error))`, `error` unchanged.

## `delivery_context`

`delivery_context<Purpose>(&self, context: &DeliveryContext<F, Purpose>, purpose: u16) -> Result<IDeliveryVerifier::DeliveryContext, EvmVerifierErrorReturn>`, private. Each form's `to_field(ToFieldParams, &form)` is unpacked irrefutably and matched, in the order below; a field of another variant returns `Err(EvmVerifierErrorReturn::ContextFieldKind { field, expected })` for the first such field in that order, a branch no form the repository declares takes and which has no unit test:

| Field | Context value | Expected variant | Member |
|---|---|---|---|
| `ChainIdentifier` | `context.chain_identifier` | `CanonicalFieldValue::Unsigned256` | `chain` as `U256::from_be_bytes` |
| `EntitlementContract` | `context.entitlement_contract` | `CanonicalFieldValue::FixedBytes20` | `entitlementContract` as `FixedBytes::from` |
| `SourceEntitlement` | `context.source_entitlement` | `CanonicalFieldValue::Unsigned256` | `sourceEntitlement` as `U256::from_be_bytes` |
| `TargetEntitlement` | `context.target_entitlement` | `CanonicalFieldValue::Unsigned256` | `targetEntitlement` as `U256::from_be_bytes` |
| `OldInterval` | `context.old_interval` | `CanonicalFieldValue::Unsigned64` | `oldInterval` |
| `NewInterval` | `context.new_interval` | `CanonicalFieldValue::Unsigned64` | `newInterval` |
| `Seller` | `context.seller` | `CanonicalFieldValue::FixedBytes20` | `seller` as `FixedBytes::from` |
| `Buyer` | `context.buyer` | `CanonicalFieldValue::FixedBytes20` | `buyer` as `FixedBytes::from` |

The remaining members: `suiteIdentifier` is `*context.suite_identifier.identifier()`, `suiteVersion` `context.suite_identifier.version()`, `assetIdentityHash` `*context.asset_identity_hash.as_bytes()`, `parameterSet` `*context.parameter_set_identifier.as_bytes()`, `purpose` the `purpose` argument, the purpose's code, and `expiry` `context.expiry`.

## `encode_scalar_word`

`encode_scalar_word(&self, scalar: P::Scalar) -> Result<U256, EvmVerifierErrorReturn>`, private: dependency call `self.pairing.encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar })`, the bytes read by `expose().as_ref()`; a length other than thirty-two returns `Err(EvmVerifierErrorReturn::ScalarLength { actual })`, `actual` the bytes' length, a branch no pairing the repository declares takes and which has no unit test; otherwise `Ok(U256::from_be_bytes)` over the thirty-two bytes.

## `mint_calldata`

`mint_calldata(&self, statement: &AlgebraicMintStatement<F, P::G1, P::G2>, challenge: P::Scalar, responses: MintResponses<P::Scalar>, first_messages: Vec<P::G2>) -> Result<Vec<u8>, EvmVerifierErrorReturn>`, private: the context by `delivery_context` with `statement.context.purpose.code()`, then each of `hpub`, `identity_element`, `buyer_keys.pk1`, `buyer_keys.pk2`, `envelope.c1`, `envelope.c2`, `envelope.d1`, and `envelope.d2` through `encode_g1` or `encode_g2` by the group of its type into `IDeliveryVerifier::MintStatement`'s member of that name, then `challenge`, `alpha`, `r`, `rho`, and `sigma` through `encode_scalar_word` into `IDeliveryVerifier::MintProof`, `secondGroupFirstMessages` the `encode_g2` bytes of `first_messages` concatenated in order; outcome `Ok` of `IDeliveryVerifier::verifyMintCall { statement, proof }.abi_encode()`; the first refusal returns, the context before the group elements before the scalars.

## `transfer_calldata`

`transfer_calldata(&self, statement: &AlgebraicTransferStatement<F, P::G1, P::G2>, challenge: P::Scalar, responses: TransferResponses<P::Scalar>, first_messages: Vec<P::G2>) -> Result<Vec<u8>, EvmVerifierErrorReturn>`, private, as `mint_calldata` over `identity_element`, `seller_keys.pk1`, `seller_keys.pk2`, `buyer_keys.pk1`, `buyer_keys.pk2`, `old_envelope.c1` through `d2`, and `new_envelope.c1` through `d2`, the scalars `challenge`, `x`, `y`, `s`, `rho`, and `sigma`, and `IDeliveryVerifier::verifyTransferCall`.

## `unsigned_transaction`

`unsigned_transaction(chain_id: u64, gas_limit: u64, verifier: Address, calldata: &[u8]) -> Vec<u8>`, private and pure, an associated function of `EvmVerifier` taking no `self`: the EIP-1559 transaction `TxEip1559 { chain_id, nonce: 0, gas_limit, max_fee_per_gas: 0, max_priority_fee_per_gas: 0, to: TxKind::Call(verifier), value: U256::ZERO, access_list: AccessList::default(), input: calldata.to_vec().into() }` returned as its `encoded_for_signing()`, the byte `0x02` and the RLP list of the nine fields, the form the oracle's `getL1Fee` takes.

## `verifier_request` and `fee_request`

`verifier_request(&self, calldata: &[u8]) -> TransactionRequest`, private: `TransactionRequest::default().with_to(self.verifier).with_input(calldata.to_vec())`. `fee_request(&self, unsigned: &[u8]) -> TransactionRequest`, private: `TransactionRequest::default().with_to(self.oracle).with_input(IGasPriceOracle::getL1FeeCall { _data: unsigned.to_vec().into() }.abi_encode())`.

## `read_chain`

`read_chain(&self, calldata: Vec<u8>) -> VerifyOnChainReturn`, private; every read runs under `self.runtime.block_on` over `self.provider`, and every transport refusal is `Err(VerifyOnChainErrorReturn::Evm(EvmVerifierErrorReturn::Rpc(error)))`, `error` unchanged; the reads in order below, the first refusal or verdict returning:

1. Verifier code: `get_code_at(self.verifier)`; a refusal returns `Rpc`; an empty result returns `Err(VerifyOnChainErrorReturn::Evm(EvmVerifierErrorReturn::NothingDeployed { address }))`, `address` the verifier's twenty bytes.
2. Verifying call: `call(self.verifier_request(&calldata))`; a refusal whose `as_error_resp()` is `Some(payload)` with `payload.as_revert_data()` `Some(data)` returns `Ok(VerifyOnChainSuccessReturn::Reverted { data: data.to_vec() })`, and any other refusal, a transport failure or a JSON-RPC error carrying no revert data, returns `Rpc`; a returned value is decoded by `bool::abi_decode` through `SolValue`, the one decode both functions' returns share, a refusal returning `Err(VerifyOnChainErrorReturn::Evm(EvmVerifierErrorReturn::ResultDecode(error)))`, `false` returning `Ok(VerifyOnChainSuccessReturn::Rejected)`, and `true` continuing.
3. L2 execution gas: `estimate_gas(self.verifier_request(&calldata))`; a refusal returns `Rpc`; the value is `l2_execution_gas`.
4. L1 data fee: `get_code_at(self.oracle)`; a refusal returns `Rpc`; an empty result returns `Ok(VerifyOnChainSuccessReturn::Accepted { cost: VerifyCost { l2_execution_gas, l1_data_fee: L1DataFee::NotApplicable } })`; otherwise `get_chain_id()`, a refusal returning `Rpc`, then `unsigned_transaction(chain_id, l2_execution_gas, self.verifier, &calldata)` handed to `call(self.fee_request(&unsigned))`, any refusal, a revert included, returning `Rpc`; a returned value that `U256::abi_decode` through `SolValue` refuses returns `ResultDecode(error)`; a fee for which `u128::try_from` fails returns `Err(VerifyOnChainErrorReturn::Evm(EvmVerifierErrorReturn::L1FeeOutOfRange { fee }))`, `fee` the big-endian bytes of the `U256`; otherwise `Ok(VerifyOnChainSuccessReturn::Accepted { cost: VerifyCost { l2_execution_gas, l1_data_fee: L1DataFee::Charged { wei } } })`.

## Ordering

The relation, the context, the group elements, the scalars, then the reads: the verifier's code, the verifying call, the gas estimate, the oracle's code, the chain identifier, and the oracle's fee; a refusal or verdict before the reads leaves the chain unread, and the response queue of a scripted chain untouched; the requests the reads send are `verifier_request` and `fee_request`, and the transaction the fee is read over is `unsigned_transaction`, each a private method a unit test calls, since a scripted chain records no request.

## Integration: callee dispositions

- `chain/create_chain_forms` entries `chain_forms_identity_survives_its_field_round_trip`, `chain_forms_entitlement_survives_its_field_round_trip`, `chain_forms_interval_survives_its_field_round_trip`, and `chain_forms_chain_identifier_survives_its_field_round_trip`: carried, restated in this node's terms: the identity forms at the entitlement contract, the seller, and the buyer reach the calldata as twenty-byte members, the entitlement forms as 256-bit members, the interval forms as 64-bit members, and the chain-identifier form as a 256-bit member, each the field its form's `to_field` yields. Route: `create_pairing`, the arm's `try_new`, `consume_pairing`, `create_chain_forms`, `consume_chain_forms`, `create_delivery_proof`, `consume_delivery_proof`, `delivery_context`, and the form's `to_field`; the route reaches no outer-edge collaborator.
- `proof/factory` entry `a_proof_the_adapter_rebuilds_from_its_components_verifies`: carried, restated in this node's terms: the components `proof_components` reads from a real proof, the challenge, the responses in witness order, and the second-group first messages the adapter's form carries, reach the calldata in that order, a mint's `alpha`, `r`, `rho`, `sigma` and a transfer's `x`, `y`, `s`, `rho`, `sigma`, then the first messages. Route: `create_pairing`, `create_delivery_proof`, `prove_mint` or `prove_transfer`, `proof_components`, `mint_calldata` or `transfer_calldata`; the route reaches no outer-edge collaborator.
- `proof/factory` entries `a_mint_proof_the_adapter_returns_verifies` and `a_transfer_proof_the_adapter_returns_verifies`: carried, restated in this node's terms: the real proofs the own entries encode are proofs the reference verifies for the statement they are formed over, and the chain's verdict on them is the deployed contract's, observed by the harness's run and not by this node. Route: `create_pairing`, `create_delivery_proof`, `prove_mint` or `prove_transfer`, `proof_components`, then `verify_on_chain`; the outer-edge collaborator is the scripted provider, the chain.
- No `encoding/factory` entry and no other `proof/factory` or `chain/create_chain_forms` entry has a route that reaches a function this node authors or its tests run; no function of this node consumes an encoder.

## Integration: own entries

- `a_real_proofs_calldata_decodes_to_its_statement_and_proof_at_the_pairings_encoding_lengths`: condition `calldata` over the statement `build_algebraic_mint_statement` or `build_algebraic_transfer_statement` builds and the components of the proof the real delivery proof adapter's `prove_mint` or `prove_transfer` returns over the witnesses `build_prove_mint_payload` or `build_prove_transfer_payload` holds, read by `proof_components`, the calldata decoded by `verifyMintCall::abi_decode` or `verifyTransferCall::abi_decode`; outcome every `bytes` member that holds a first-group element is the length of the first group's precompile encoding of the case's curve, `64` on BN254 and `128` on BLS12-381, every member that holds a second-group element `128` on BN254 and `256` on BLS12-381, `secondGroupFirstMessages` three second-group encodings long, in the order `hpub`, `d1`, `d2` for a mint and `pk2`, `d1`, `d2` for a transfer, where the pairing declares `VerifierGroupArithmetic::FirstGroupOnly` and empty where it declares `BothGroups`, and every scalar member the thirty-two-byte word of the encoded scalar; variation every member of `PAIRING_CONCRETES`, both relations, so a first-group member holding a second-group encoding, a scalar narrowed, or first messages dropped or reordered fails; edge that must survive the composition: the lengths are the precompile encodings' the real pairing emits, stated by curve and never read from the pairing or the declaration, and every real encoder, forms, and proof component flows through the ABI the interface file fixes.
- `an_accepted_verdict_and_its_costs_reach_the_caller_for_a_real_proof`: condition `verify_on_chain` over the same statement and proof, on a provider scripted to answer the verifier's code non-empty, the verifying call `true`, the gas estimate a value other than every default, and the oracle's code first empty and then non-empty with a chain identifier and a fee; outcome `Ok(VerifyOnChainSuccessReturn::Accepted { cost })` with `l2_execution_gas` the scripted gas and `l1_data_fee` `NotApplicable` for the empty oracle code and `Charged { wei }` with the scripted fee for the non-empty; variation every member of `PAIRING_CONCRETES`, both relations, and both oracle states; edge that must survive the composition: the cost is read from the chain in the order the reads are made, so a swapped read or a fee read where the oracle has no code fails.
- `a_rejecting_chain_returns_rejected_for_a_real_proof`: condition as the accepted entry with the verifying call answering `false`; outcome `Ok(VerifyOnChainSuccessReturn::Rejected)` and the scripted answers after the verifying call unread; variation every member of `PAIRING_CONCRETES` and both relations; edge that must survive the composition: a rejected delivery is a verdict and reads no cost.
- `a_reverting_chain_returns_the_revert_data_for_a_real_proof`: condition as the accepted entry with the verifying call refused by a JSON-RPC error carrying the hex data `0x08c379a0`; outcome `Ok(VerifyOnChainSuccessReturn::Reverted { data })` with `data` `[0x08, 0xc3, 0x79, 0xa0]` and the answers after unread; variation every member of `PAIRING_CONCRETES` and both relations; edge that must survive the composition: the revert data is the chain's, carried whole, and a revert is a verdict, never an `Rpc` error.
- `a_real_proof_of_the_other_relation_is_refused_before_any_chain_read`: condition `verify_on_chain` over a real mint statement and the real proof `prove_transfer` returned, and over a real transfer statement and the proof `prove_mint` returned, the scripted provider holding one answer; outcome `Err(VerifyOnChainErrorReturn::RelationMismatch { statement, proof })` with the relations the case names and the scripted answer unread; variation every member of `PAIRING_CONCRETES` and both orders; edge that must survive the composition: the refusal is the family's flavor, decided from the real proof's components before the chain is read, and names no concrete.

## Integration: private surface

The chain of real functions `create_pairing`, the arm's concrete through its `try_new`, `create_hash_to_scalar`, the selected concrete's `try_new`, `create_encoding`, `create_chain_forms`, `create_delivery_proof`, the selected concrete's `try_new`, `prove_mint`, `prove_transfer`, `proof_components`, `create_random_source` for `RandomSourceKind::OperatingSystem`, `EvmVerifier::try_new_mocked`, `verify_on_chain`, `calldata`, `mint_calldata`, `transfer_calldata`, `delivery_context`, `encode_scalar_word`, `unsigned_transaction`, `verifier_request`, `fee_request`, and `read_chain`; the cases are every member of `PAIRING_CONCRETES` crossed with the selections `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declare `HashToScalarIdentifier::Keccak256V1`, `DECLARED_ENCODING_SELECTIONS`, `CHAIN_FORMS_CONCRETES`, and `DELIVERY_PROOF_CONCRETES`; the outer-edge collaborators are the chain, a provider scripted through `Asserter`, and the operating system's generator running real; the observable result is the `VerifyOnChainReturn`, the decoded calldata, and the scripted queue's remaining answers; the entries proven are every own entry above and the carried entries.
