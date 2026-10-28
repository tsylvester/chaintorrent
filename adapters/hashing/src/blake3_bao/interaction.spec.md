# blake3_bao interactions

`Blake3BaoCommitment` is the commitment family's first concrete: the adapter over `blake3` and `bao`. Its fallible constructor `try_new` is its only producer; `Blake3BaoCommitment::DECLARATION` names the commitment-scheme identifier it sits under, its chunk length, its key length, its adapter version, and the interface version it implements, readable before any instance exists. It implements `ICommitmentAdapter`.

## `Blake3BaoCommitment::try_new`

`Blake3BaoCommitment::try_new(params: Blake3BaoCommitmentConstructorParams) -> Blake3BaoCommitmentTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| admitted | any params | none | none | `Ok(Blake3BaoCommitment)`; the error arm has no branch |

## `Blake3BaoCommitment::DECLARATION`

The inherent constant `CommitmentDeclaration { identifier: CommitmentSchemeIdentifier::Blake3BaoV1, chunk_length: BLAKE3_BAO_CHUNK_LENGTH, key_length: BLAKE3_BAO_KEY_LENGTH, adapter_version: 1, interface_version: COMMITMENT_INTERFACE_VERSION }`.

## `ICommitmentAdapter::declaration`

`Blake3BaoCommitment::declaration(&self) -> CommitmentDeclaration`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| declared | any adapter | none | none | `Blake3BaoCommitment::DECLARATION` |

## Shared terms

- The chunk count of a content length `L` is `1` when `L` is zero and `L.div_ceil(BLAKE3_BAO_CHUNK_LENGTH)` otherwise.
- The expected outboard length is `OUTBOARD_HEADER_LENGTH + OUTBOARD_PARENT_LENGTH × (chunk count − 1)`.
- The expected length of chunk `i` is `min(BLAKE3_BAO_CHUNK_LENGTH, L − i × BLAKE3_BAO_CHUNK_LENGTH)`.
- A subtree at offset `o` of length `n` is a leaf when `n ≤ BLAKE3_BAO_CHUNK_LENGTH`, and otherwise splits at `left_subtree_len(n)` into a left child at `o` and a right child at `o + left_subtree_len(n)`.
- The mode is `Mode::Hash` unkeyed and `Mode::KeyedHash(&key)` keyed.

## Leaf and parent values

- A leaf's chaining value is a `Hasher`, `Hasher::new()` unkeyed and `Hasher::new_keyed(&key)` keyed, given `set_input_offset(o)`, then `update` with the leaf's bytes, then `finalize_non_root`.
- A parent's chaining value is `merge_subtrees_non_root(left, right, mode)`.
- The root of a single-chunk object is that same `Hasher`'s `finalize` over the content with no offset set, and the root of a larger object is `merge_subtrees_root` of the top split's two children.
- Every keyed hasher is zeroized after its last call.

## Key admission

Shared by `commit`, `verify`, and `verify_chunk` when the key is `CommitmentKey::Keyed(key)`: decision `<[u8; 32]>::try_from(key.expose().as_slice())`, before any hashing; outcome on failure `Err({Method}ErrorReturn::KeyLengthMismatch { expected: BLAKE3_BAO_KEY_LENGTH, actual })`, `actual` the exposed key's length; on success the copied key array is zeroized when the method returns, on every branch.

## Outboard admission

Shared by `verify`, `extract_path`, and `respond`: condition the outboard holds fewer than `OUTBOARD_HEADER_LENGTH` bytes; outcome `Err({Method}ErrorReturn::OutboardTruncated { length })`, `length` the outboard's byte length; otherwise the declared content length is `u64::from_le_bytes` of the header.

## `ICommitmentAdapter::commit`

`Blake3BaoCommitment::commit(params: CommitParams, payload: CommitPayload<'_>) -> CommitReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| unkeyed | `payload.key` is `Unkeyed` | none | `bao::encode::outboard(payload.content)` once | `Ok(CommitSuccessReturn { root, outboard })`, `root` from `Root::try_new` over the returned hash's bytes and `outboard` the returned bytes moved into `Outboard`; a `Root::try_new` refusal returns `Err(CommitErrorReturn::Root(error))` with the error unchanged, an arm no input reaches since an all-zero BLAKE3 output is computationally unreachable, so it has no unit test |
| keyed | `payload.key` is `Keyed` and the key admits | `u64::try_from(payload.content.len())`, failing as `Err(CommitErrorReturn::Blake3Bao(Blake3BaoCommitmentErrorReturn::ContentLengthUnrepresentable { length }))`, an arm no input reaches on a supported target, so it has no unit test | the leaf and parent values over the tree in the keyed mode, written as the outboard layout states, header first and parents in pre-order | `Ok(CommitSuccessReturn { root, outboard })`, `root` through `Root::try_new` as above |

## `ICommitmentAdapter::verify`

`Blake3BaoCommitment::verify(params: VerifyParams, payload: VerifyPayload<'_>) -> VerifyReturn`

Admissions and lengths, in order, each the first failing check returning:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| key rejected | the key does not admit | the key admission | none | `Err(VerifyErrorReturn::KeyLengthMismatch { expected, actual })` |
| outboard truncated | the outboard holds fewer than `OUTBOARD_HEADER_LENGTH` bytes | the outboard admission | none | `Err(VerifyErrorReturn::OutboardTruncated { length })` |
| content length | the declared length differs from `payload.content.len()` as a `u64` | the comparison | none | `Err(VerifyErrorReturn::ContentLengthMismatch { declared, actual })` |
| outboard length | the outboard's length differs from the expected outboard length | the comparison | none | `Err(VerifyErrorReturn::OutboardLengthMismatch { expected, actual })` |

Tree, in order after the lengths pass:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| root | the root computed from the top split's recorded children, or from the content itself for a single-chunk object, differs from `payload.root` | the comparison | the leaf and parent values | `Err(VerifyErrorReturn::RootMismatch)` |
| parent | walking the parent nodes in pre-order, a parent's two recorded children merge to a chaining value different from the one its parent recorded | the comparison | `merge_subtrees_non_root` | `Err(VerifyErrorReturn::ParentMismatch { offset, length })` naming that subtree |
| chunk | walking leaves left to right, a leaf's computed chaining value differs from the one its parent recorded | the comparison | the leaf values | `Err(VerifyErrorReturn::ChunkMismatch { chunk_index })` |
| verified | every check passes | none | none | `Ok(VerifySuccessReturn)` |

Conversions: the content length to `u64` and every offset to `usize` through `TryFrom`, failing as the concrete's own `ContentLengthUnrepresentable` or `OffsetUnrepresentable`, arms no input reaches on a supported target, with no unit test.

## `ICommitmentAdapter::extract_path`

`Blake3BaoCommitment::extract_path(params: ExtractPathParams, payload: ExtractPathPayload<'_>) -> ExtractPathReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| outboard truncated | the outboard holds fewer than `OUTBOARD_HEADER_LENGTH` bytes | the outboard admission | none | `Err(ExtractPathErrorReturn::OutboardTruncated { length })` |
| outboard length | the outboard's length differs from the expected outboard length for the declared length | the comparison | none | `Err(ExtractPathErrorReturn::OutboardLengthMismatch { expected, actual })` |
| index | `payload.chunk_index` is not less than the chunk count | the comparison | none | `Err(ExtractPathErrorReturn::ChunkIndexOutOfRange { chunk_index, chunk_count })` |
| extracted | every check passes | the descent | none | `Ok(ExtractPathSuccessReturn { path })` |

The descent: from the root, at each parent node the sibling of the side holding the chunk is appended and the descent moves to that side, the right child's node lying past every parent node of the left subtree in pre-order; a single-chunk object yields an empty path. Offsets convert as `verify`'s do.

## `ICommitmentAdapter::verify_chunk`

`Blake3BaoCommitment::verify_chunk(params: VerifyChunkParams, payload: VerifyChunkPayload<'_>) -> VerifyChunkReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| key rejected | the key does not admit | the key admission | none | `Err(VerifyChunkErrorReturn::KeyLengthMismatch { expected, actual })` |
| index | `payload.chunk_index` is not less than the chunk count of `payload.content_length` | the comparison | none | `Err(VerifyChunkErrorReturn::ChunkIndexOutOfRange { chunk_index, chunk_count })` |
| chunk length | the chunk's length differs from the expected length of that chunk | the comparison | none | `Err(VerifyChunkErrorReturn::ChunkLengthMismatch { expected, actual })` |
| path length | the path's length differs from the number of splits on the route from the root to the chunk | the comparison | none | `Err(VerifyChunkErrorReturn::PathLengthMismatch { expected, actual })` |
| root | the leaf's chaining value folded upward with the path's siblings, deepest first, the top merge by `merge_subtrees_root`, or the content's root for a single-chunk object, differs from `payload.root` | the comparison | the leaf and parent values | `Err(VerifyChunkErrorReturn::RootMismatch)` |
| verified | every check passes | none | none | `Ok(VerifyChunkSuccessReturn)` |

The chunk's length converts as `verify`'s content length does.

## `ICommitmentAdapter::challenge`

`Blake3BaoCommitment::challenge(params: ChallengeParams, payload: ChallengePayload) -> ChallengeReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| drawn | any payload | `u64::from_le_bytes(payload.uniform)` modulo the chunk count of `payload.content_length` | none | `Ok(ChallengeSuccessReturn { chunk_index })`; the error arm has no variant |

## `ICommitmentAdapter::respond`

`Blake3BaoCommitment::respond(params: RespondParams, payload: RespondPayload<'_>) -> RespondReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| outboard truncated | the outboard holds fewer than `OUTBOARD_HEADER_LENGTH` bytes | the outboard admission | none | `Err(RespondErrorReturn::OutboardTruncated { length })` |
| content length | the declared length differs from `payload.content.len()` as a `u64` | the comparison | none | `Err(RespondErrorReturn::ContentLengthMismatch { declared, actual })` |
| outboard length | the outboard's length differs from the expected outboard length | the comparison | none | `Err(RespondErrorReturn::OutboardLengthMismatch { expected, actual })` |
| index | `payload.chunk_index` is not less than the chunk count | the comparison | none | `Err(RespondErrorReturn::ChunkIndexOutOfRange { chunk_index, chunk_count })` |
| answered | every check passes | none | the descent as `extract_path` descends | `Ok(RespondSuccessReturn { chunk, path })`, `chunk` the content's bytes of that chunk |

Conversions as `verify`'s.

Ordering: no method hashes before its admissions pass; the same inputs always yield the same outcome; no method mutates or retains a payload borrow.

## Integration: own entries

Each entry is named and proven by the private integration block of the same name.

- `commit_under_a_key_leaves_the_borrowed_key_intact`: condition `commit` is called with `CommitmentKey::Keyed` borrowing a living `Secret<Vec<u8>>`; outcome after the call the secret's `expose()` equals the bytes it held before the call, so the zeroization of the key copy and the keyed hashers wipes only their own copies; variation the key is the reference key, whose bytes are not all zero; the failure that must survive composition is the method's zeroization reaching the caller's secret.
- `committed_outboard_verifies_its_content`: condition `verify` is given the content, root, and outboard `commit` returned for it; outcome `Ok(VerifySuccessReturn)`, while the same root and outboard with one content byte changed in the third chunk return `Err(VerifyErrorReturn::ChunkMismatch { chunk_index: 2 })`; variation the reference content of 3073 bytes, four chunks under three parent nodes, intact and with one byte changed.
- `extracted_paths_verify_every_chunk`: condition `verify_chunk` is given, for every chunk index of a committed object, that chunk's bytes and the path `extract_path` returned for that index; outcome every call returns `Ok(VerifyChunkSuccessReturn)`, while chunk 1's path with chunk 0's bytes of the same length returns `Err(VerifyChunkErrorReturn::RootMismatch)`; variation the reference content of 3073 bytes, whose chunks differ, so a path that ignores the index fails.
- `a_challenge_response_verifies_against_the_root`: condition `respond` answers the chunk index `challenge` chose from fixed uniform bytes, over the committed content and outboard, and `verify_chunk` checks the response against the committed root; outcome `Ok(VerifyChunkSuccessReturn)`, while a response computed over an outboard with one parent byte changed returns `Err(VerifyChunkErrorReturn::RootMismatch)`; variation the intact outboard and the outboard altered in its first parent node, the root's, whose left child is the sibling on the challenged chunk's path, the false-possession case of SW-04.
- `keyed_paths_verify_only_under_their_key`: condition a path extracted from a keyed commitment is checked with `verify_chunk` under the same key and unkeyed; outcome `Ok(VerifyChunkSuccessReturn)` under the key and `Err(VerifyChunkErrorReturn::RootMismatch)` unkeyed; variation the mode, the content and chunk index fixed.

## Integration: private surface

The chain of real functions is `build_blake3_bao_commitment` through `Blake3BaoCommitment::try_new`, then `Blake3BaoCommitment::commit`, `verify`, `extract_path`, `verify_chunk`, `challenge`, and `respond`, which run `bao::encode::outboard`, `blake3::Hasher`, `blake3::hazmat`, `Zeroize::zeroize`, and `Root::try_new` for real; the key is a `domain` `Secret` whose `expose` runs for real; no outer-edge collaborator is mocked; the observable results are the methods' returns and the borrowed key's bytes after the call.
