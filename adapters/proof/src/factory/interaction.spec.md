# `create_delivery_proof` — interaction spec

Branch contract for the `factory` module of the `proof` crate: the delivery-proof family's construction point, which selects the concrete the configuration names, admits it against the resolved envelope's algebra, the verifier form the resolved pairing declares, and the delivery-statement version the hash-card names, constructs it in that form over borrowed collaborators, and hands it to a consumer generic over the family's trait. Each branch states condition, decision, dependency call, and the exact return outcome.

## `create_delivery_proof<'a, P: IPairingArithmetic, E: IEncoderAdapter, F: IChainForms, C: IDeliveryProofConsumer<P, F>>(deps: &CreateDeliveryProofDeps<'a, P, E, C>, params: CreateDeliveryProofParams, payload: CreateDeliveryProofPayload) -> CreateDeliveryProofReturn<C::Output>`

The decision is a `match` on `params.concrete`, one arm per `DeliveryProofConcrete` variant, exhaustive so a variant with no arm fails to compile. The `SchnorrFs` arm takes the branches below.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| unsupported algebra | `SchnorrFsDeliveryProof::<'_, P, E, F>::DECLARATION.algebras` does not contain `params.algebra` | slice membership, read before any construction | none | `Err(CreateDeliveryProofErrorReturn::UnsupportedEnvelopeAlgebra)`, nothing constructed and the consumer not called; `EnvelopeAlgebra` has the one variant the concrete declares, so no input takes this branch until a further algebra exists, and it has no unit test |
| unsupported verifier form | the algebra is declared and `DECLARATION.verifier_forms` does not contain `params.verifier_form` | slice membership | none | `Err(CreateDeliveryProofErrorReturn::UnsupportedVerifierForm)`, nothing constructed and the consumer not called; the concrete declares both forms, so no input takes this branch until a further form exists, and it has no unit test |
| unsupported statement version | the algebra and form are declared and `DECLARATION.statement_versions` does not contain `params.statement_version` | slice membership | none | `Err(CreateDeliveryProofErrorReturn::UnsupportedStatementVersion)`, nothing constructed and the consumer not called |
| constructor refused | every requirement is declared and `try_new` returns `Err(error)` | the concrete's constructor runs once | `SchnorrFsDeliveryProof::try_new(SchnorrFsDeliveryProofConstructorParams { pairing: deps.pairing, hash_to_scalar: deps.hash_to_scalar, encoder: deps.encoder, random: deps.random, verifier_form: params.verifier_form })` | `Err(CreateDeliveryProofErrorReturn::SchnorrFs(error))`, the refusal unchanged, the consumer not called; the concrete's weight tag is admitted, so no input takes this branch and it has no unit test |
| admitted | `try_new` returns `Ok(adapter)` | the adapter moves into the consumer's payload | `deps.consumer.consume_delivery_proof(ConsumeDeliveryProofParams, ConsumeDeliveryProofPayload { adapter })`, exactly once | `Ok(CreateDeliveryProofSuccessReturn { output })` holding the consumer's output |

## Ordering and invariants

- The algebra check, then the form check, then the version check, then construction in the required form, then the consumer — each decided before the next runs.
- `params.concrete` selects; `params.algebra`, `params.verifier_form`, and `params.statement_version` admit; `params.verifier_form` also configures the constructed concrete.
- `payload` carries nothing and is not read.
- The deps' borrows are copied into the constructor params; the deps are unchanged, and the caller keeps each collaborator for the credential KEM and the key agreement.
- The consumer is generic over `IDeliveryProofAdapter` whose `Pairing` is the caller's pairing type and whose `Forms` are the consumer's chain forms; it never names the concrete.

