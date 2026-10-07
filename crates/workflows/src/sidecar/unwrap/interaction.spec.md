# unwrap interactions

`unwrap_piece_group_key` recovers a piece-group key from a parameter set's sidecar entry: it encodes the derivation context through the injected encoder, derives the wrapping key through the injected key-derivation adapter under the `WrappingKey` purpose from the encapsulated value's key material over the encoded context, and returns the byte-wise XOR of the wrapped key and the wrapping key. It is the wrap's inverse under the same derivation.

## `unwrap_piece_group_key`

`unwrap_piece_group_key<E: IEncoderAdapter>(deps: &UnwrapPieceGroupKeyDeps<'_, E>, params: UnwrapPieceGroupKeyParams, payload: UnwrapPieceGroupKeyPayload<'_>) -> UnwrapPieceGroupKeyReturn`

First, `DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams)` is unpacked irrefutably — its error arm is `Infallible` — then `deps.encoder.encode(EncodeParams { description: &description }, payload.context)` is called exactly once. Then the branches, in order:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| encoding refused | the `encode` call returns `Err(error)` | none — the refusal is the branch | `encode` once; the KDF is never called | `Err(UnwrapPieceGroupKeyErrorReturn::Encoding(error))`, the refusal unchanged |
| key derivation refused | `deps.kdf.derive_key(DeriveKeyParams { purpose: DerivationPurpose::WrappingKey, length: PIECE_GROUP_KEY_LENGTH }, DeriveKeyPayload { key_material: payload.encapsulated.key_material(), context: &encoded })`, called once, returns `Err(error)` | none — the refusal is the branch | `derive_key` once | `Err(UnwrapPieceGroupKeyErrorReturn::KeyDerivation(error))`, the refusal unchanged |
| wrapping key of another length | the derivation succeeds and the derived key's exposed length is not `PIECE_GROUP_KEY_LENGTH` | the length comparison, made before any byte is combined | none further | `Err(UnwrapPieceGroupKeyErrorReturn::WrappingKeyLength { expected, actual })`, `expected` is `PIECE_GROUP_KEY_LENGTH` and `actual` the derived key's |
| unwrapped | the derived key's length is `PIECE_GROUP_KEY_LENGTH` | the comparison above | none | `Ok(UnwrapPieceGroupKeySuccessReturn { piece_group_key })`, `piece_group_key` built by the crate-private `PieceGroupKey::from_array` from a `Secret<[u8; 32]>` filled by XORing `payload.wrapped.as_bytes()` with the wrapping key |

Ordering and lifecycle: encoding precedes derivation, the length check precedes the XOR, and the buffer is moved into its `Secret` as soon as it is filled. The derived wrapping key is held in its returned `Secret`, which drops — and so zeroizes — when the function returns on every branch. The encapsulated value, the context, and the wrapped key are borrowed and unchanged; `params` carries no control and is not read; the same payload always yields the same outcome.

Invariant: for any encapsulated value, context, and key, unwrapping the wrap's output returns the key.

