# `factory` — interaction spec

Branch contract for the `factory` module of the `kem` crate: the credential KEM family's construction point, selecting the concrete the composition names, admitting it against the KEM identifier and the identity scope the suite requires, and handing it to a consumer generic over `ICredentialKemAdapter` for the caller's pairing. Each branch states condition, decision, dependency call, and the exact return outcome.

## `create_kem<'a, P: IPairingArithmetic, C: IKemConsumer<P>>(deps: &CreateKemDeps<'a, P, C>, params: CreateKemParams, payload: CreateKemPayload) -> CreateKemReturn<C::Output>`

The decision is a `match` on `params.concrete`, one arm per `KemConcrete` variant — `Bb1DepthOne` — exhaustive, so a variant with no arm fails to compile. The arm runs the same contract against the concrete it names.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| unsupported identifier | `Bb1DepthOneKem::<'_, P>::DECLARATION.identifier` is not `params.identifier` | equality, read before any construction | none | `Err(CreateKemErrorReturn::UnsupportedKemIdentifier)`; nothing is constructed and the consumer is not called. `KemIdentifier` has the one variant the concrete declares, so no input takes this branch until a further identifier exists, and it has no unit test |
| unsupported scope | the identifier matches and `DECLARATION.identity_scopes` does not contain `params.scope` | `contains`, read before any construction | none | `Err(CreateKemErrorReturn::UnsupportedIdentityScope)`; nothing is constructed and the consumer is not called. The concrete declares both scopes, so no input takes this branch until a concrete declaring fewer exists, and it has no unit test |
| constructor refused | both checks pass and `Bb1DepthOneKem::try_new(Bb1DepthOneKemConstructorParams { pairing: deps.pairing, hash_to_scalar: deps.hash_to_scalar })` returns `Err(error)` | the constructor's result | `try_new` once | `Err(CreateKemErrorReturn::Bb1DepthOne(error))`, the refusal unchanged, the consumer not called. The concrete's tag is admitted, so no input takes this branch and it has no unit test |
| admitted | `try_new` returns `Ok(adapter)` | none | `deps.consumer.consume_kem(ConsumeKemParams, ConsumeKemPayload { adapter, scope: params.scope })`, exactly once, the consumer reading `K::DECLARATION` | `Ok(CreateKemSuccessReturn { output })` holding the consumer's output |

The `Bb1DepthOne` variant of `CreateKemErrorReturn` carries `Bb1DepthOneKemTryNewErrorReturn`, that concrete's constructor error, in the return union unchanged; each further concrete's constructor refusal is its own variant.

## Ordering and edges

- `params.concrete` selects the concrete; `params.identifier` and `params.scope` admit or refuse it. `payload` carries nothing and is not read.
- The identifier check precedes the scope check and both precede construction in every arm: a refused concrete is never constructed, and `deps.consumer` is never touched.
- The deps' borrows are copied into the constructor params and the deps are unchanged.
- The consumer is generic over `K: ICredentialKemAdapter<Pairing = P>` for the caller's pairing `P`; it is instantiated for the concrete the arm constructs and never names the concrete itself.

