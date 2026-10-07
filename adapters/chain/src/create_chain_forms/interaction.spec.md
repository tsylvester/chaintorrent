# `create_chain_forms` — interaction spec

Branch contract for the `create_chain_forms` module of the `chain` crate: the family's forms construction point — it selects the forms concrete `params.concrete` names, admits it only when its declared identifier is `params.identifier`, constructs it, and hands it to the consumer in `deps`, which is generic over `IChainForms` and never names the concrete. Each branch states condition, decision, dependency call, and the exact return outcome.

## `create_chain_forms<C: IChainFormsConsumer>(deps: &CreateChainFormsDeps<C>, params: CreateChainFormsParams, payload: CreateChainFormsPayload) -> CreateChainFormsReturn<C::Output>`

Decision: a `match` on `params.concrete`, one arm per `ChainFormsConcrete` variant, exhaustive so a variant with no arm fails to compile.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| unsupported identifier | the named concrete's `DECLARATION.identifier` is not `params.identifier` | equality, read before any construction | none | `Err(CreateChainFormsErrorReturn::UnsupportedChainFormsIdentifier)`; nothing is constructed and the consumer is not called |
| admitted | the named concrete's `DECLARATION.identifier` is `params.identifier` | the same comparison | `EvmForms::try_new(EvmFormsConstructorParams)`, exactly once, its success destructured irrefutably because its error arm is uninhabited; then `deps.consumer.consume_chain_forms(ConsumeChainFormsParams, ConsumeChainFormsPayload { forms })`, exactly once | `Ok(CreateChainFormsSuccessReturn { output })`, holding the consumer's output |

`ChainFormsIdentifier` has the one variant the EVM forms concrete declares, so no input takes the unsupported-identifier branch until a further identifier exists; the branch has no unit test.

`CreateChainFormsErrorReturn::Evm` carries the EVM concrete's uninhabited constructor error type in the return union, so no branch produces it.

## Ordering and invariants

- Within an arm: the identifier check, then construction, then the consumer. `params.concrete` selects the concrete; `params.identifier` admits it.
- `payload` carries nothing and is not read. `deps.consumer` is invoked at most once per call, only on the admitted branch.
- The consumer receives the forms as `F: IChainForms` and reads `F::DECLARATION`; nothing in this module names a forms concrete other than in the selecting arm.

