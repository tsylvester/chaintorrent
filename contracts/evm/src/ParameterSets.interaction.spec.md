# `ParameterSets` — interaction spec

Branch contract for the `ParameterSets` contract of `contracts/evm`: `ParameterSets` is a parameter-set registry whose branch is selected by its inputs, the caller's address, and its stored records, never by the digest's or the material's meaning — `material` and `identityScope` are stored and returned unread. The contract's boundary is its public ABI, proven by the interface, guard, and unit blocks of this node; the Rust integration elements do not apply to a contract. Each branch states condition and exact outcome.

## Construction

`ParameterSets()` stores `msg.sender` as the immutable registry address and records nothing; the deployer is the only address that may register or retire a parameter set.

## `registerParameterSet`

`registerParameterSet(bytes32 id, bytes32 asset, bytes32 suiteId, bytes calldata material, uint8 identityScope)` — each guard below is a comparison made before any state is written.

### The caller is not the registry

Condition `msg.sender` is not the registry address; outcome revert `CallerNotRegistry(caller)`.

### `id` is zero

Condition `id` is zero; outcome revert `ParameterSetIdUnset()`.

### `asset` is zero

Condition `asset` is zero; outcome revert `AssetUnset()`.

### `suiteId` is zero

Condition `suiteId` is zero; outcome revert `SuiteUnset()`.

### `material` is empty

Condition `material` has length zero; outcome revert `MaterialEmpty()`.

### `id` is already recorded

Condition the record of `id` has a nonzero `asset`, recorded live or retired; outcome revert `ParameterSetAlreadyRegistered(id)`.

### Register success

Outcome the record of `id` is stored with `id`, `asset`, `suiteId`, `material`, `identityScope`, and `live` true, and `ParameterSetRegistered(id, asset, suiteId, identityScope)` is emitted.

## `retireParameterSet`

`retireParameterSet(bytes32 id)` — each guard below is a comparison made before any state is written.

### The caller is not the registry

Condition `msg.sender` is not the registry address; outcome revert `CallerNotRegistry(caller)`.

### `id` is not recorded

Condition the record of `id` has a zero `asset`; outcome revert `ParameterSetNotRegistered(id)`.

### `id` is already retired

Condition the record's `live` is false; outcome revert `ParameterSetAlreadyRetired(id)`.

### Retire success

Outcome the record's `live` becomes false, every other member is unchanged, and `ParameterSetRetired(id)` is emitted.

## `parameterSet`

`parameterSet(bytes32 id)` returns the record of `id`, every member zero for an `id` never recorded; it reads state only.

## `isParameterSetLive`

`isParameterSetLive(bytes32 id)` returns the record's `live`, false for an `id` never recorded; it reads state only.

## Ordering

The caller decision precedes every other decision; the argument decisions precede the duplicate decision; every decision precedes the state write; a revert leaves every record unchanged.

## Boundaries

The contract calls no other contract; the decision whether no live entitlement remains under a set precedes a call to `retireParameterSet` and is made by the caller.
