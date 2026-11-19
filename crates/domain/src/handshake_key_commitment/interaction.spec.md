# handshake_key_commitment interactions

`HandshakeKeyCommitment` is an owned value type: a fallible constructor `try_new` is its only producer, and the read accessors `as_bytes` and `scheme` are its only views. There are no dependency calls; the only decision is a byte-level check on the commitment, and the scheme it holds is a `SignatureSchemeIdentifier` admitted by that type's own constructor, so no branch examines it.

## `HandshakeKeyCommitment::try_new`

`HandshakeKeyCommitment::try_new(params: HandshakeKeyCommitmentConstructorParams) -> HandshakeKeyCommitmentTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| all zero | every byte of `params.bytes` is `0` | `params.bytes.iter().all(…)` over the byte equal to `0` | none | `Err(HandshakeKeyCommitmentTryNewErrorReturn::AllZero)` |
| admitted | some byte of `params.bytes` is nonzero | the same check | none | `Ok(HandshakeKeyCommitment { bytes, scheme })`, the array and the scheme moved from the params, the scheme unexamined because `SignatureSchemeIdentifier` is admitted by its own constructor |

Ordering: the single check fully decides the outcome; the same params always yield the same outcome.

## `HandshakeKeyCommitment::as_bytes`

`HandshakeKeyCommitment::as_bytes(&self) -> &[u8; HANDSHAKE_KEY_COMMITMENT_LENGTH]`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| read | none | none | none | `&self.bytes`, a shared reference to the held array — no copy, no side effect |

## `HandshakeKeyCommitment::scheme`

`HandshakeKeyCommitment::scheme(&self) -> SignatureSchemeIdentifier`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| read | none | none | none | `self.scheme`, the held scheme, copied because the type is `Copy` — no side effect |

## Invariants

- Every `HandshakeKeyCommitment` holds 32 bytes, not all zero — the length is a fact of the params type, never a runtime check, and the all-zero value is what an unassigned `bytes32` storage slot reads as, so it names no key the Binding record holds — and one admitted `SignatureSchemeIdentifier`.
- `try_new` is the only producer; `Clone` copies only an already-admitted value, so no admitted form exists outside the constructor's invariants.

## Integration: callee dispositions

The interaction spec of `domain/signature_scheme_identifier` declares no integration entries, so no callee entry passes through a function of this node or ends at one of its tests.

## Integration: own entries

- `the_all_zero_commitment_is_refused_whatever_its_scheme`: condition `HandshakeKeyCommitment::try_new` called through the crate's public surface with 32 zero bytes and a scheme admitted by its own constructor; outcome `Err(HandshakeKeyCommitmentTryNewErrorReturn::AllZero)`; the input variation is two schemes, so a refusal conditioned on the scheme passes for one and fails for the other; the edge that must survive composition is the refusal of the zero bytes under the smallest and the largest admitted scheme.
- `an_admitted_commitment_reads_back_its_bytes_and_its_scheme`: condition `HandshakeKeyCommitment::try_new` called through the crate's public surface with nonzero bytes and a scheme admitted by `SignatureSchemeIdentifier::try_new`; outcome `Ok` whose `as_bytes()` is the bytes handed and whose `scheme().identifier()` is the identifier the scheme was admitted with; the input variation is bytes of 32 distinct values and a scheme differing from the builder's default, so a copy that reorders the bytes or supplies its own scheme fails.

## Integration: private surface

None; the entries are proven at the crate's boundary, and this node carries no private integration test.

## Integration: public surface

An outside caller invokes `HandshakeKeyCommitment::try_new` with a `HandshakeKeyCommitmentConstructorParams` built from the crate's public surface under the `mocks` feature, and observes the `Err` arm's `AllZero` or the `Ok` arm's `as_bytes()` and `scheme()`; the entries proven are `the_all_zero_commitment_is_refused_whatever_its_scheme` and `an_admitted_commitment_reads_back_its_bytes_and_its_scheme`.
