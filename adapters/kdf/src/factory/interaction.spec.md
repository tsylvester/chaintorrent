# factory interactions

`create_key_derivation` is the key-derivation family's construction point: it selects the concrete `params.concrete` names, admits it only when its `DECLARATION.identifier` is the `params.identifier` the hash-card requires, and returns it behind `Box<dyn IKeyDerivationAdapter>` with its declaration. It reads no hash-card and no configuration — the composition resolver passes both as typed values — and it does not derive; derivation is the concrete's.

## `create_key_derivation`

`create_key_derivation(deps: &CreateKeyDerivationDeps, params: CreateKeyDerivationParams, payload: CreateKeyDerivationPayload) -> CreateKeyDerivationReturn`

The decision is a `match` on `params.concrete`, one arm per `KdfConcrete` variant, exhaustive so a variant with no arm fails to compile. Within each arm:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| unsupported identifier | the named concrete's `DECLARATION.identifier` is not `params.identifier` | equality, read before any construction | none | `Err(CreateKeyDerivationErrorReturn::UnsupportedKdfIdentifier)`; nothing is constructed. `KdfIdentifier` has the one variant the BLAKE3 concrete declares, so no input takes this branch until a further identifier exists, and it has no unit test |
| admitted | the named concrete's declared identifier is `params.identifier` | the equality above | the concrete's `try_new` with its fieldless constructor params, exactly once, its success destructured irrefutably because its error arm is uninhabited | `Ok(CreateKeyDerivationSuccessReturn { adapter: Box::new(kdf) })`, whose adapter reports the concrete's `DECLARATION` |

`CreateKeyDerivationErrorReturn::Blake3Keyed` carries the BLAKE3 concrete's constructor error type `Infallible` in the return union; the error arm is uninhabited, so no branch produces it and none is constructed for it.

`params.concrete` selects and `params.identifier` admits; `deps` and `payload` carry nothing and are not read.

