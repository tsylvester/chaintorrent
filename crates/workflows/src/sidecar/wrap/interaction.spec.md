# wrap interactions

`wrap_piece_group_key` wraps a piece-group key under a parameter set's
wrapping key: it encodes the derivation context through the injected encoder,
derives the wrapping key through the injected key-derivation adapter under the
`WrappingKey` purpose from the encapsulated value's key material over the
encoded context, and returns the byte-wise XOR of the piece-group key and the
wrapping key. It carries no tag — the sidecar's Bao root authenticates every
entry — and it produces no variable-length wrapped value.

## `wrap_piece_group_key`

`wrap_piece_group_key<E: IEncoderAdapter>(deps: &WrapPieceGroupKeyDeps<'_, E>, params: WrapPieceGroupKeyParams, payload: WrapPieceGroupKeyPayload<'_>) -> WrapPieceGroupKeyReturn`

First, `DerivationContextDescription::try_new(DerivationContextDescriptionConstructorParams)`
is unpacked irrefutably — its error arm is `Infallible` — then
`deps.encoder.encode(EncodeParams { description: &description }, payload.context)`
is called exactly once. Then the branches, in order:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| encoding refused | the `encode` call returns `Err(error)` | none — the refusal is the branch | `encode` once; the KDF is never called | `Err(WrapPieceGroupKeyErrorReturn::Encoding(error))`, the refusal unchanged |
| key derivation refused | `deps.kdf.derive_key(DeriveKeyParams { purpose: DerivationPurpose::WrappingKey, length: PIECE_GROUP_KEY_LENGTH }, DeriveKeyPayload { key_material: payload.encapsulated.key_material(), context: &encoded })`, called once, returns `Err(error)` | none — the refusal is the branch | `derive_key` once | `Err(WrapPieceGroupKeyErrorReturn::KeyDerivation(error))`, the refusal unchanged |
| wrapping key of another length | the derivation succeeds and the derived key's exposed length is not the piece-group key's length | the length comparison, made before any byte is combined | none further | `Err(WrapPieceGroupKeyErrorReturn::WrappingKeyLength { expected, actual })`, `expected` the piece-group key's length and `actual` the derived key's |
| wrapped | the derived key's length is `PIECE_GROUP_KEY_LENGTH` | the comparison above | none | `Ok(WrapPieceGroupKeySuccessReturn { wrapped })`, `wrapped` the 32-byte array filled by XORing the admitted piece-group key with the wrapping key; no variable-length wrapped value enters trusted code |

Ordering and lifecycle: encoding precedes derivation, and the length check
precedes the XOR. The derived wrapping key is held in its returned `Secret`,
which drops — and so zeroizes — when the function returns on every branch. The
piece-group key and the encapsulated value are borrowed and unchanged;
`params` carries no control and is not read; the same payload always yields
the same outcome.
