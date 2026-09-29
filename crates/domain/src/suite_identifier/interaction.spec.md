# suite_identifier interactions

`SuiteIdentifier` is an owned value type: a fallible constructor `try_new`
is its only producer, and the read accessors `identifier` and `version` are
its only views. There are no dependency calls; the only decisions are a
byte-level check on the identifier and an equality check on the version,
taken in that order.

## `SuiteIdentifier::try_new`

`SuiteIdentifier::try_new(params: SuiteIdentifierConstructorParams) -> SuiteIdentifierTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| all-zero identifier | every byte of `params.identifier` is `0` | `params.identifier.iter().all(…)` over the byte equal to `0` | none | `Err(SuiteIdentifierTryNewErrorReturn::AllZeroIdentifier)` |
| zero version | the identifier passes and `params.version` is `0` | the equality check | none | `Err(SuiteIdentifierTryNewErrorReturn::ZeroVersion)` |
| admitted | some byte of `params.identifier` is nonzero and `params.version` is nonzero | the same two checks | none | `Ok(SuiteIdentifier { identifier, version })`, both moved from the params |

Ordering: the identifier's check precedes the version's; the same params
always yield the same outcome.

## `SuiteIdentifier::identifier`

`SuiteIdentifier::identifier(&self) -> &[u8; SUITE_IDENTIFIER_LENGTH]`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| read | none | none | none | `&self.identifier`, a shared reference to the held array — no copy, no side effect |

## `SuiteIdentifier::version`

`SuiteIdentifier::version(&self) -> u16`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| read | none | none | none | `self.version`, the held version |

## Invariants

- Every `SuiteIdentifier` holds a 32-byte identifier, not all zero, and a
  nonzero version — the length is a fact of the params type, never a
  runtime check, and the all-zero identifier and zero version are what
  unassigned storage slots read as, so neither names a suite the Registry
  registered.
- `try_new` is the only producer; `Clone` copies only an already-admitted
  value, so no admitted form exists outside the constructor's invariants.
