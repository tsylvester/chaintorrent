# blake3_keyed interactions

`Blake3KeyedKdf` is the key-derivation family's first concrete: the adapter
over `blake3`'s derive-key mode. Its fallible constructor `try_new` is its
only producer; `Blake3KeyedKdf::DECLARATION` names the KDF identifier it sits
under, its adapter version, and the interface version it implements, readable
before any instance exists. It implements `IKeyDerivationAdapter`.

## `Blake3KeyedKdf::try_new`

`Blake3KeyedKdf::try_new(params: Blake3KeyedKdfConstructorParams) -> Blake3KeyedKdfTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| admitted | any params | none | none | `Ok(Blake3KeyedKdf)`; the error arm has no branch |

## `Blake3KeyedKdf::DECLARATION`

The inherent constant
`KdfDeclaration { identifier: KdfIdentifier::Blake3KeyedV1, adapter_version: 1, interface_version: KDF_INTERFACE_VERSION }`.

## Purpose mapping

A `match` on `params.purpose`, exhaustive, each variant to its context string
constant: `WrappingKey` to `BLAKE3_KEYED_WRAPPING_KEY_CONTEXT`,
`PublisherRoot` to `BLAKE3_KEYED_PUBLISHER_ROOT_CONTEXT`, `AssetRoot` to
`BLAKE3_KEYED_ASSET_ROOT_CONTEXT`, `MasterScalar` to
`BLAKE3_KEYED_MASTER_SCALAR_CONTEXT`, `IdentityBases` to
`BLAKE3_KEYED_IDENTITY_BASES_CONTEXT`, `CapsuleRandomness` to
`BLAKE3_KEYED_CAPSULE_RANDOMNESS_CONTEXT`, `PieceGroupKey` to
`BLAKE3_KEYED_PIECE_GROUP_KEY_CONTEXT`, and `PlaintextRootKey` to
`BLAKE3_KEYED_PLAINTEXT_ROOT_KEY_CONTEXT`.

## `IKeyDerivationAdapter::derive_key`

`Blake3KeyedKdf::derive_key(params: DeriveKeyParams, payload: DeriveKeyPayload<'_>) -> DeriveKeyReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| prefix overflow | `u64::try_from(payload.key_material.expose().len())` fails | the conversion, before any hashing | none | `Err(DeriveKeyErrorReturn::Blake3Keyed(Blake3KeyedKdfDeriveKeyErrorReturn::KeyMaterialLengthExceedsPrefix { length }))`; `usize` is at most 64 bits on every supported target, so no input takes this branch and it has no unit test |
| derived | the length converts | none further | `Hasher::new_derive_key` with the purpose's context string, then `update` with the length's 8 big-endian bytes, `update` with the exposed key material, and `update` with `payload.context`, in that order, each once; then `finalize_xof` once and `fill` once over a zero-initialized buffer of `params.length` bytes; then `zeroize` on the output reader and on the hasher | `Ok(DeriveKeySuccessReturn { key })`, the filled buffer moved into a `Secret` without copy |

Ordering: the length conversion precedes the hasher; the prefix, the key
material, and the context are absorbed in that order; the reader and the
hasher are zeroized before the buffer is moved into the `Secret`; the same
params and payload always yield the same key; a `params.length` of zero
yields an empty key.
