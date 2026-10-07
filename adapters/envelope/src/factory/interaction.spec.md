# `factory` — interaction spec

Branch contract for the `factory` module of the `envelope` crate: the key-agreement family's construction point, selecting the concrete the composition names, admitting it against the key-agreement identifier the hash-card names and the envelope algebra the suite's delivery proof requires, and handing it to a consumer generic over `IKeyAgreementAdapter` for the caller's pairing. Each branch states condition, decision, dependency call, and the exact return outcome.

## `create_key_agreement<'a, P: IPairingArithmetic, E: IEncoderAdapter, C: IKeyAgreementConsumer<P>>(deps: &CreateKeyAgreementDeps<'a, P, E, C>, params: CreateKeyAgreementParams, payload: CreateKeyAgreementPayload) -> CreateKeyAgreementReturn<C::Output>`

The decision is a `match` on `params.concrete`, one arm per `KeyAgreementConcrete` variant — `PairingElGamal` — exhaustive, so a variant with no arm fails to compile. The arm runs the same contract against the concrete it names.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| unsupported identifier | `PairingElGamalKeyAgreement::<'_, P, E>::DECLARATION.identifier` is not `params.identifier` | equality, read before any construction | none | `Err(CreateKeyAgreementErrorReturn::UnsupportedKeyAgreementIdentifier)`; nothing is constructed and the consumer is not called. `KeyAgreementIdentifier` has the one variant the concrete declares, so no input takes this branch until a further identifier exists, and it has no unit test |
| unsupported algebra | the identifier matches and `DECLARATION.algebra` is not `params.algebra` | equality, read before any construction | none | `Err(CreateKeyAgreementErrorReturn::UnsupportedEnvelopeAlgebra)`; nothing is constructed and the consumer is not called. `EnvelopeAlgebra` has the one variant the concrete declares, so no input takes this branch until a further algebra exists, and it has no unit test |
| constructor refused | both checks pass and `PairingElGamalKeyAgreement::try_new(PairingElGamalKeyAgreementConstructorParams { pairing: deps.pairing, hash_to_scalar: deps.hash_to_scalar, encoder: deps.encoder, random: deps.random })` returns `Err(error)` | the constructor's result | `try_new` once | `Err(CreateKeyAgreementErrorReturn::PairingElGamal(error))`, the refusal unchanged, the consumer not called. The concrete's tags are admitted, so no input takes this branch and it has no unit test |
| admitted | `try_new` returns `Ok(adapter)` | none | `deps.consumer.consume_key_agreement(ConsumeKeyAgreementParams, ConsumeKeyAgreementPayload { adapter })`, exactly once, the consumer reading `K::DECLARATION` | `Ok(CreateKeyAgreementSuccessReturn { output })` holding the consumer's output |

The `PairingElGamal` variant of `CreateKeyAgreementErrorReturn` carries `PairingElGamalKeyAgreementTryNewErrorReturn`, that concrete's constructor error, in the return union unchanged; each further concrete's constructor refusal is its own variant.

## Ordering and edges

- `params.concrete` selects the concrete; `params.identifier` and `params.algebra` admit or refuse it. `payload` carries nothing and is not read.
- The identifier check precedes the algebra check and both precede construction in every arm: a refused concrete is never constructed, and `deps.consumer` is never touched.
- The deps' borrows are copied into the constructor params and the deps are unchanged.
- The consumer is generic over `K: IKeyAgreementAdapter<Pairing = P>` for the caller's pairing `P`; it is instantiated for the concrete the arm constructs and never names the concrete itself.

