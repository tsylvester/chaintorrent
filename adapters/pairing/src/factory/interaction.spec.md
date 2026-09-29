# `factory` — interaction spec

Branch contract for the `factory` module of the `pairing` crate: the pairing family's construction point, admitting the concrete the composition names against the chain's declared precompile encodings and handing it to a consumer generic over `IPairingAdapter`. Each branch states condition, decision, dependency call, and the exact return outcome.

## `create_pairing<C: IPairingConsumer>(deps: &CreatePairingDeps<C>, params: CreatePairingParams, payload: CreatePairingPayload) -> CreatePairingReturn<C::Output>`

The decision is a `match` on `params.concrete`, one arm per `PairingConcrete` variant — `Bn254Arkworks`, `Bn254Halo2curves`, `Bls12381Arkworks`, `Bls12381Halo2curves` — exhaustive, so a variant with no arm fails to compile. Each arm runs the same two-branch contract against its concrete.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| unsupported encoding | the arm's concrete's `DECLARATION.precompile_encoding` is not in `params.supported_encodings` | `contains`, read before any construction | none | `Err(CreatePairingErrorReturn::UnsupportedPrecompileEncoding)`; nothing is constructed and the consumer is not called |
| admitted | the arm's concrete's `DECLARATION.precompile_encoding` is in `params.supported_encodings` | `contains` | the concrete's `try_new` with its fieldless constructor params, exactly once, its success destructured irrefutably because its error arm is uninhabited; then `deps.consumer.consume_pairing(ConsumePairingParams, ConsumePairingPayload { adapter, declaration })` with the concrete's `DECLARATION`, exactly once | `Ok(CreatePairingSuccessReturn { output })` holding the consumer's output |

Each concrete's own variant of `CreatePairingErrorReturn` — `Bn254Arkworks(Infallible)`, `Bn254Halo2curves(Infallible)`, `Bls12381Arkworks(Infallible)`, `Bls12381Halo2curves(Infallible)` — carries that concrete's constructor error in the return union unchanged; because every constructor error is `Infallible`, uninhabited, no branch produces one.

## Ordering and edges

- `params.concrete` selects the concrete; `params.supported_encodings` admits or refuses it. `payload` carries nothing and is not read.
- The encoding check precedes construction in every arm: a refused concrete is never constructed, and `deps.consumer` is never touched.
- The consumer is generic over `P: IPairingAdapter`; it is instantiated inside the arm for the concrete the arm constructs and never names the concrete itself.
