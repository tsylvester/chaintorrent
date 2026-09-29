# derivation_context interactions

`DerivationContext` is an owned value type: a fallible constructor `try_new`
is its only producer, and the read accessors `asset`, `deployment`, `suite`,
`parameter_set`, `group_index`, and `geometry` are its only views. Its one
decision is the cross-field rule between the group index and the geometry,
read through `GroupIndex::value` and `PieceGeometry::group_count`; every
other component arrives as an admitted instance of its own type and is held
unchecked.

## `DerivationContext::try_new`

`DerivationContext::try_new(params: DerivationContextConstructorParams) -> DerivationContextTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| group index out of range | `params.group_index.value() >= params.geometry.group_count()` | the comparison | `GroupIndex::value` and `PieceGeometry::group_count`, once each | `Err(DerivationContextTryNewErrorReturn::GroupIndexOutOfRange { group_index, group_count })`, holding the two values compared |
| admitted | the group index is below the group count | the same comparison | the same two reads | `Ok(DerivationContext { asset, deployment, suite, parameter_set, group_index, geometry })`, every component moved from the params |

Ordering: a single comparison decides the outcome; the same params always
yield the same outcome.

## `DerivationContext::asset`, `deployment`, `suite`, `parameter_set`, `group_index`, `geometry`

`DerivationContext::asset(&self) -> &AssetIdentity`
`DerivationContext::deployment(&self) -> &DeploymentIdentity`
`DerivationContext::suite(&self) -> &SuiteIdentifier`
`DerivationContext::parameter_set(&self) -> &ParameterSetIdentifier`
`DerivationContext::group_index(&self) -> &GroupIndex`
`DerivationContext::geometry(&self) -> &PieceGeometry`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| read | none | none | none | a shared reference to the held component — no copy, no side effect |

## Invariants

- Every `DerivationContext` holds six admitted components and a group index
  below its geometry's group count.
- `try_new` is the only producer; `Clone` copies only an already-admitted
  value, so no admitted form exists outside the constructor.
