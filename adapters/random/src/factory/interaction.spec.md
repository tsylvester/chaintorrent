# `factory` — interaction spec

Branch contract for `create_random_source` in the `factory` module of the `random` crate. Each branch states condition, decision, dependency call, and the exact return outcome.

## `create_random_source(deps: &CreateRandomSourceDeps, params: CreateRandomSourceParams, payload: CreateRandomSourcePayload) -> CreateRandomSourceReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| operating system | `params.kind` is `RandomSourceKind::OperatingSystem` | a `match` on `params.kind` | `OsRandomSource::try_new(OsRandomSourceConstructorParams)`, exactly once | `Ok(CreateRandomSourceSuccessReturn { adapter: Box::new(source), declaration: OsRandomSource::DECLARATION })` |

## Invariants and edges

- The operating-system constructor's error arm is uninhabited (`Result<_, Infallible>`), so its success is destructured irrefutably and this branch has no failure outcome. `CreateRandomSourceErrorReturn::OperatingSystem` nevertheless carries its error type in the return union, so each concrete's constructor error has a variant.
- `params.kind` selects the concrete; `deps` and `payload` carry nothing and are not read.
- The `match` is exhaustive over `RandomSourceKind`: a kind with no branch fails to compile, which is how adding a concrete forces its factory branch to be written.
