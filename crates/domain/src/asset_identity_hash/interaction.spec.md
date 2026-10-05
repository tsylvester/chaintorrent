# asset_identity_hash interactions

`AssetIdentityHash` is an owned value type: the fallible constructor `try_new`
is its only producer, and the read accessor `as_bytes` is its only view.
There are no dependency calls; the only decision is a byte-level check on the
params.

## `AssetIdentityHash::try_new`

`AssetIdentityHash::try_new(params: AssetIdentityHashConstructorParams) -> AssetIdentityHashTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| all zero | every byte of `params.bytes` is `0` | `params.bytes.iter().all(…)` over the byte equal to `0` | none | `Err(AssetIdentityHashTryNewErrorReturn::AllZero)` |
| admitted | some byte of `params.bytes` is nonzero | the same check | none | `Ok(AssetIdentityHash { bytes })`, the array moved from the params |

## `AssetIdentityHash::as_bytes`

`AssetIdentityHash::as_bytes(&self) -> &[u8; ASSET_IDENTITY_HASH_LENGTH]`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| read | none | none | none | `&self.bytes`, a shared reference to the held array — no copy, no side effect |

## Invariants and ordering

- Ordering: the single check fully decides the outcome; the same params
  always yield the same outcome, so a refusal is deterministic across every
  process.
- Every `AssetIdentityHash` holds exactly `ASSET_IDENTITY_HASH_LENGTH` (32)
  bytes, not all zero.
- `try_new` is the only producer; `Clone` copies only an already-admitted
  value, so no admitted form exists outside the constructor's invariants.
- The params type carries exactly 32 bytes, so the length is a fact of the
  type and never a runtime check.
