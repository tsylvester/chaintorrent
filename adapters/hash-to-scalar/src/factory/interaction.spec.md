# factory interactions

`create_hash_to_scalar` is the hash-to-scalar family's construction point: it
selects the concrete `params.concrete` names, admits it only when its
`DECLARATION.identifier` is the `params.identifier` the hash-card requires, and
returns it behind `Box<dyn IHashToScalarAdapter<S>>` for the scalar type `S`
the caller names, together with its declaration. It reads no hash-card and no
configuration — the composition resolver passes both as typed values — and it
does not hash; hashing is the concrete's.

## `create_hash_to_scalar`

`create_hash_to_scalar<S: ISampleUniformScalar + Clone>(deps: &CreateHashToScalarDeps, params: CreateHashToScalarParams, payload: CreateHashToScalarPayload) -> CreateHashToScalarReturn<S>`

The decision is a `match` on `params.concrete`, one arm per `HashToScalarConcrete`
variant, exhaustive so a variant with no arm fails to compile. Within each arm:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| unsupported identifier | the named concrete's `DECLARATION.identifier` is not `params.identifier` | equality, read before any construction | none | `Err(CreateHashToScalarErrorReturn::UnsupportedHashToScalarIdentifier)`; nothing is constructed. `HashToScalarIdentifier` has the one variant the Keccak-256 concrete declares, so no input takes this branch until a further identifier exists, and it has no unit test |
| admitted | the named concrete's declared identifier is `params.identifier` | the equality above | the concrete's `try_new` with its fieldless constructor params, exactly once, its success destructured irrefutably because its error arm is uninhabited | `Ok(CreateHashToScalarSuccessReturn { adapter: Box::new(hasher), declaration })` with the concrete's `DECLARATION`, the box coerced to `Box<dyn IHashToScalarAdapter<S>>` |

`CreateHashToScalarErrorReturn::Keccak256` carries the Keccak-256 concrete's
constructor error type `Infallible` in the return union; the error arm is
uninhabited, so no branch produces it and none is constructed for it.

`params.concrete` selects and `params.identifier` admits; `S` fixes the scalar
type the returned adapter produces; `deps` and `payload` carry nothing and are
not read.
