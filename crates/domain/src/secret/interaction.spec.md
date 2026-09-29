# `secret` — interaction spec

Branch contract for the `secret` module of the `domain` crate. One branch per operation; each branch states condition, decision, dependency call, and the exact return outcome.

## `Secret::try_new(params: SecretConstructorParams<T>) -> SecretTryNewReturn<T>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| construct | any params | none | none | `Ok(Secret)` holding `params.value`, moved without copy |

The error arm has no branch: moving a value into the wrapper has no failure, so `Infallible` is uninhabited.

## `Secret::expose(&self) -> &T`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| expose | a living secret | none | none | a shared reference to the held value; no copy, no side effect |

## `Drop::drop(&mut self)`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| drop | the secret's lifetime ends — scope end, move into a consumer that drops it, or unwinding | none | `Zeroize::zeroize` on the held value, exactly once | the held value's memory is zeroized before release |

## Invariants

- `Secret<T>` implements none of `core::fmt::Debug`, `core::fmt::Display`, `Clone`, or `Copy`.
- `Secret<T>` cannot implement a serialization trait: the `domain` crate carries no serialization dependency.
- `Secret::try_new` is the only producer of `Secret<T>`.
- The held value is reachable only through `expose`; the `value` field is `pub(super)`, so no code outside the `secret` module can read it.
