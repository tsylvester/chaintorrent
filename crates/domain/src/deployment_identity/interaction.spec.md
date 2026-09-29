# deployment_identity interactions

`DeploymentIdentity` is an owned value type: a fallible constructor `try_new`
is its only producer, and the read accessor `as_bytes` is its only view.
There are no dependency calls; the only decision is a byte-level check on the
params.

## `DeploymentIdentity::try_new`

`DeploymentIdentity::try_new(params: DeploymentIdentityConstructorParams) -> DeploymentIdentityTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| all zero | every byte of `params.bytes` is `0` | `params.bytes.iter().all(…)` over the byte equal to `0` | none | `Err(DeploymentIdentityTryNewErrorReturn::AllZero)` |
| admitted | some byte of `params.bytes` is nonzero | the same check | none | `Ok(DeploymentIdentity { bytes })`, the array moved from the params |

## `DeploymentIdentity::as_bytes`

`DeploymentIdentity::as_bytes(&self) -> &[u8; DEPLOYMENT_IDENTITY_LENGTH]`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| read | none | none | none | `&self.bytes`, a shared reference to the held array — no copy, no side effect |

## Invariants

- Every `DeploymentIdentity` holds exactly 32 bytes, not all zero — the length
  is a fact of the params type, never a runtime check, and the all-zero value
  is what an unassigned `bytes32` storage slot reads as, so it names no
  deployment the Registry assigned.
- `try_new` is the only producer; `Clone` copies only an already-admitted
  value, so no admitted form exists outside the constructor's invariants.
