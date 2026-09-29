# group_index interactions

`GroupIndex` is an owned value type: a fallible constructor `try_new` is its
only producer, and the read accessor `value` is its only view. There are no
dependency calls and no decisions; every `u64` is an index a deployment can
carry, so the error arm has no branch.

## `GroupIndex::try_new`

`GroupIndex::try_new(params: GroupIndexConstructorParams) -> GroupIndexTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| admitted | any params | none | none | `Ok(GroupIndex { value })`, the index moved from `params.value` |

Ordering: a single branch decides the outcome; the same params always yield
the same outcome. The error arm has no branch.

## `GroupIndex::value`

`GroupIndex::value(&self) -> u64`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| read | none | none | none | `self.value`, a copy of the held index — no side effect |

## Invariants

- Every `GroupIndex` holds exactly one `u64`, read back unchanged by `value`.
- `try_new` is the only producer; `Clone` copies only an already-admitted
  value, so no admitted form exists outside the constructor.
- The index's bound against a deployment's group count is not this type's —
  `domain/derivation_context` holds the geometry and refuses an index outside
  it.
