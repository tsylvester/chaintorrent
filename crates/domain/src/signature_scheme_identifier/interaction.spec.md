# signature_scheme_identifier interactions

`SignatureSchemeIdentifier` is an owned value type: a fallible constructor `try_new` is its only producer, and the read accessor `identifier` is its only view. There are no dependency calls; the only decision is an equality check on the identifier.

## `SignatureSchemeIdentifier::try_new`

`SignatureSchemeIdentifier::try_new(params: SignatureSchemeIdentifierConstructorParams) -> SignatureSchemeIdentifierTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| zero identifier | `params.identifier` is `0` | the equality check | none | `Err(SignatureSchemeIdentifierTryNewErrorReturn::ZeroIdentifier)` |
| admitted | `params.identifier` is nonzero | the same check | none | `Ok(SignatureSchemeIdentifier { identifier })`, moved from the params |

Ordering: the same params always yield the same outcome.

## `SignatureSchemeIdentifier::identifier`

`SignatureSchemeIdentifier::identifier(&self) -> u8`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| read | none | none | none | `self.identifier`, the held value — no side effect |

## Invariants

- Every `SignatureSchemeIdentifier` holds a nonzero `u8` — the zero value is what an unassigned `uint8` storage slot reads as, so it names no declared scheme.
- `try_new` is the only producer; `Clone` and `Copy` copy only an already-admitted value, so no admitted form exists outside the constructor's invariants.
