# blake3_keyed interactions

`Blake3KeyedKdf` is the key-derivation family's first concrete: the adapter over `blake3`'s derive-key mode. Its fallible constructor `try_new` is its only producer; `Blake3KeyedKdf::DECLARATION` names the KDF identifier it sits under, its adapter version, and the interface version it implements, readable before any instance exists. It implements `IKeyDerivationAdapter`.

## `Blake3KeyedKdf::try_new`

`Blake3KeyedKdf::try_new(params: Blake3KeyedKdfConstructorParams) -> Blake3KeyedKdfTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| admitted | any params | none | none | `Ok(Blake3KeyedKdf)`; the error arm has no branch |

## `Blake3KeyedKdf::DECLARATION`

The inherent constant `KdfDeclaration { identifier: KdfIdentifier::Blake3KeyedV1, adapter_version: 1, interface_version: KDF_INTERFACE_VERSION }`.

## `IKeyDerivationAdapter::declaration`

`Blake3KeyedKdf::declaration(&self) -> KdfDeclaration`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| declared | any adapter | none | none | `Blake3KeyedKdf::DECLARATION` |

## Purpose mapping

A `match` on `params.purpose`, exhaustive, each variant to its context string constant: `WrappingKey` to `BLAKE3_KEYED_WRAPPING_KEY_CONTEXT`, `PublisherRoot` to `BLAKE3_KEYED_PUBLISHER_ROOT_CONTEXT`, `AssetRoot` to `BLAKE3_KEYED_ASSET_ROOT_CONTEXT`, `MasterScalar` to `BLAKE3_KEYED_MASTER_SCALAR_CONTEXT`, `IdentityBases` to `BLAKE3_KEYED_IDENTITY_BASES_CONTEXT`, `CapsuleRandomness` to `BLAKE3_KEYED_CAPSULE_RANDOMNESS_CONTEXT`, `PieceGroupKey` to `BLAKE3_KEYED_PIECE_GROUP_KEY_CONTEXT`, and `PlaintextRootKey` to `BLAKE3_KEYED_PLAINTEXT_ROOT_KEY_CONTEXT`.

## `IKeyDerivationAdapter::derive_key`

`Blake3KeyedKdf::derive_key(params: DeriveKeyParams, payload: DeriveKeyPayload<'_>) -> DeriveKeyReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| prefix overflow | `u64::try_from(payload.key_material.expose().len())` fails | the conversion, before any hashing | none | `Err(DeriveKeyErrorReturn::Blake3Keyed(Blake3KeyedKdfDeriveKeyErrorReturn::KeyMaterialLengthExceedsPrefix { length }))`; `usize` is at most 64 bits on every supported target, so no input takes this branch and it has no unit test |
| derived | the length converts | none further | `Hasher::new_derive_key` with the purpose's context string, then `update` with the length's 8 big-endian bytes, `update` with the exposed key material, and `update` with `payload.context`, in that order, each once; then `finalize_xof` once and `fill` once over a zero-initialized buffer of `params.length` bytes; then `zeroize` on the output reader and on the hasher | `Ok(DeriveKeySuccessReturn { key })`, the filled buffer moved into a `Secret` without copy |

Ordering: the length conversion precedes the hasher; the prefix, the key material, and the context are absorbed in that order; the reader and the hasher are zeroized before the buffer is moved into the `Secret`; the same params and payload always yield the same key; a `params.length` of zero yields an empty key.

## Integration: own entries

Each entry is named and proven by the private integration block of the same name.

- `derive_key_leaves_the_borrowed_key_material_intact`: condition `derive_key` is called with a living `Secret<Vec<u8>>` borrowed in the payload; outcome after the call the secret's `expose()` equals the bytes it held before the call, so the zeroization of the hasher and the output reader, which absorbed those bytes, wipes only their own copies; variation the key material is the reference key material, whose bytes are not all zero, so a zeroized secret reads differently from an intact one; edge that must survive the composition: the hasher's zeroization reaching the caller's secret.
- `derive_key_returns_a_key_that_outlives_the_key_material`: condition `derive_key` returns `Ok` and the key material's secret is then dropped, its drop zeroizing its value; outcome the returned key's `expose()` still equals the derived bytes; variation the key material's secret is dropped after the call and before the key is read, so a returned key that shares storage with the key material reads as zeros; edge that must survive the composition: the returned `Secret` aliasing any input's storage.

## Integration: private surface

The chain of real functions is `build_blake3_keyed_kdf` through `Blake3KeyedKdf::try_new`, then `Blake3KeyedKdf::derive_key`, which runs `blake3::Hasher`, `blake3::OutputReader`, and `Zeroize::zeroize` for real and moves the buffer into a `Secret` through `Secret::try_new`; the input and output secrets are `domain` `Secret` values whose `expose` and drop run for real; no outer-edge collaborator is mocked; the observable result is the bytes read through `expose()` of the key material before and after the call and of the returned key after the key material is dropped.

