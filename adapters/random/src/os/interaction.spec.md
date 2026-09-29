# `os` — interaction spec

Branch contract for the `os` module of the `random` crate: the operating-system randomness concrete. Each branch states condition, decision, dependency call, and the exact return outcome.

## `OsRandomSource::try_new(params: OsRandomSourceConstructorParams) -> OsRandomSourceTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| construct | any params | none | none | `Ok(OsRandomSource)` |

The error arm has no branch: the operating-system source takes no configuration, so `Infallible` is uninhabited.

## `OsRandomSource::DECLARATION`

An inherent constant, readable from the type before any instance exists:

`RandomSourceDeclaration { source: RandomSourceKind::OperatingSystem, adapter_version: 1, interface_version: RANDOM_SOURCE_INTERFACE_VERSION }`

## `OsRandomSource::fill_bytes(&self, params: FillBytesParams, payload: FillBytesPayload) -> FillBytesReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| drawn | `getrandom::fill` returns `Ok(())` over a zero-initialized buffer of `payload.length` bytes | the fill result | `getrandom::fill`, exactly once | `Ok(FillBytesSuccessReturn { bytes })`, the filled buffer moved into a `Secret` without copy |
| generator failed | `getrandom::fill` returns `Err(error)` | the fill result | `getrandom::fill`, exactly once | `Err(FillBytesErrorReturn::OperatingSystem(OsRandomSourceFillBytesErrorReturn::OperatingSystem(error)))`, the error unchanged; the buffer, already moved into a `Secret`, is zeroized when it drops |

## Ordering and edges

- The buffer is moved into a `Secret` **after** `getrandom::fill` returns and **before** its result is inspected, so the buffer is zeroized on both branches.
- A `payload.length` of zero takes the drawn branch with an empty buffer.
- `params` carries no control and is not read.
