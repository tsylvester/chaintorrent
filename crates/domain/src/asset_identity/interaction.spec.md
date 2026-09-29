# asset_identity interactions

`AssetIdentity` is an owned value type: a fallible constructor `try_new` is its
only producer, and the read accessors `name` and `version` are its only views.
There are no dependency calls; every decision is a byte-level check on the
params.

## `AssetIdentity::try_new`

`AssetIdentity::try_new(params: AssetIdentityConstructorParams) -> AssetIdentityTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| empty name | `params.name.is_empty()` | the emptiness check | none | `Err(AssetIdentityTryNewErrorReturn::EmptyName)` |
| name byte outside visible ASCII | the name is non-empty and some byte of `params.name.bytes()` fails `is_ascii_graphic` | the first such byte by index | none | `Err(AssetIdentityTryNewErrorReturn::NameByteOutsideVisibleAscii { index, byte })` for the lowest such index |
| empty version | the name passes and `params.version.is_empty()` | the emptiness check | none | `Err(AssetIdentityTryNewErrorReturn::EmptyVersion)` |
| version byte outside visible ASCII or the separator | the name passes, the version is non-empty, and some byte of `params.version.bytes()` fails `is_ascii_graphic` or equals `ASSET_COORDINATE_SEPARATOR` | the first such byte by index, scanned once left to right | none | `Err(AssetIdentityTryNewErrorReturn::VersionByteOutsideVisibleAscii { index, byte })` when that byte fails `is_ascii_graphic`; `Err(AssetIdentityTryNewErrorReturn::VersionContainsSeparator { index })` when it is the separator |
| admitted | every check passes | none further | none | `Ok(AssetIdentity { name, version })`, both strings moved from the params without copy |

## `AssetIdentity::name`

`AssetIdentity::name(&self) -> &str`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| read | none | none | none | `&self.name`, a shared reference to the held string — no copy, no side effect |

## `AssetIdentity::version`

`AssetIdentity::version(&self) -> &str`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| read | none | none | none | `&self.version`, a shared reference to the held string — no copy, no side effect |

## Invariants and ordering

- Ordering: the name's checks precede the version's; within each string the
  lowest offending index decides; the same params always yield the same
  outcome, so a refusal is deterministic across every process.
- A version byte that is both non-graphic and the separator is impossible
  (`b'@'` is graphic), so each offending index maps to exactly one error
  variant.
- Every `AssetIdentity` holds a non-empty name and a non-empty version of
  visible ASCII (`0x21..=0x7E`), the version free of `@`; the join
  `name@version` therefore splits at its last `@` into exactly one name and
  one version.
- `try_new` is the only producer; `Clone` copies only an already-admitted
  value, so no admitted form exists outside the constructor's invariants.
