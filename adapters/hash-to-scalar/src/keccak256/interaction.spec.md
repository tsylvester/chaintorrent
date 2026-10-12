# `keccak256` — interaction spec

Branch contract for the `keccak256` module of the `hash-to-scalar` crate: the Keccak-256 concrete hashing a length-prefixed domain tag and a message and reducing the digest modulo the group order through the scalar type's own sampling bound. Each branch states condition, decision, dependency call, and the exact return outcome.

## `Keccak256HashToScalar::try_new(params: Keccak256HashToScalarConstructorParams) -> Keccak256HashToScalarTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| constructed | any params | none | none | `Ok(Keccak256HashToScalar)`; the error arm is uninhabited and has no branch |

## `Keccak256HashToScalar::DECLARATION`

The inherent constant `HashToScalarDeclaration { identifier: HashToScalarIdentifier::Keccak256V1, adapter_version: 1, interface_version: HASH_TO_SCALAR_INTERFACE_VERSION }`, readable before any instance exists.

## `IHashToScalarAdapter::<S>::hash_to_scalar(&self, params: HashToScalarParams<'_>, payload: HashToScalarPayload<'_>) -> HashToScalarReturn<S>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| tag too long | `u8::try_from(params.tag.as_bytes().len())` fails | the conversion, first | none | `Err(HashToScalarErrorReturn::Keccak256(Keccak256HashToScalarErrorReturn::TagLengthExceedsPrefix { length }))`, `length` the tag's length; a `DomainTag` holds at most 255 bytes, so no input takes this branch and it has no unit test |
| sampling input too short | the prefix conversion passes and `S::UNIFORM_BYTES_LENGTH.checked_sub(KECCAK256_DIGEST_LENGTH)` is `None` | the subtraction, before hashing | none | `Err(HashToScalarErrorReturn::Keccak256(Keccak256HashToScalarErrorReturn::UniformLengthBelowDigest { uniform_length: S::UNIFORM_BYTES_LENGTH, digest_length: KECCAK256_DIGEST_LENGTH }))` |
| sampling refused | both checks pass and `S::sample_from_uniform_bytes` returns `Err(error)` | the sampler's result | the hashing calls below, then `S::sample_from_uniform_bytes` once | `Err(HashToScalarErrorReturn::Sampling(error))`, the error unchanged; the input is exactly `S::UNIFORM_BYTES_LENGTH` bytes |
| hashed | both checks pass and sampling succeeds | all checks pass | `Keccak256::new()`, then `update` with the one prefix byte, `update` with `params.tag.as_bytes()`, and `update` with `payload.message`, in that order, each once, then `finalize` once; a buffer of `S::UNIFORM_BYTES_LENGTH` zero bytes receives the digest in its last `KECCAK256_DIGEST_LENGTH` bytes and is moved into a `Secret` by `let Ok(uniform) = Secret::try_new(SecretConstructorParams { value: buffer });`; then `S::sample_from_uniform_bytes(SampleUniformScalarParams, SampleUniformScalarPayload { uniform })` once | `Ok(HashToScalarSuccessReturn { scalar })`, `scalar` a clone of the sampled `Secret`'s exposed value, the `Secret` then dropping and zeroizing its copy |

## Ordering and invariants

- The prefix conversion and the length subtraction precede the hasher; the prefix, the tag, and the message are absorbed in that order. The same tag and message always yield the same scalar for one scalar type.
- The digest is read as a big-endian integer, zero-extended at the front to `S::UNIFORM_BYTES_LENGTH`, and reduced modulo the group order by the scalar type's own sampler — no field arithmetic is duplicated here, and no pairing library or use's tag is named.

## Integration: callee dispositions

- `domain_tag` entry `a_tag_of_the_maximum_length_reads_back_through_the_crate_root` (the admitted tag read-back): carried — restated in this node's terms: the bytes `hash_to_scalar` absorbs after the length byte are the held tag's `as_bytes()`. Route: the crate barrel `lib.rs`, `domain_tag::provides`, `DomainTag::try_new`, `DomainTag::as_bytes`, then `Keccak256HashToScalar::hash_to_scalar`; the route reaches no outer-edge collaborator.
- `domain_tag` entry `a_tag_one_byte_over_the_maximum_is_refused_whole_through_the_crate_root` (the refusal that reaches the caller whole): carried — restated in this node's terms: a refused tag yields no `DomainTag`, so no `HashToScalarParams` exists and `hash_to_scalar` is not reached. Route: the crate barrel `lib.rs`, `domain_tag::provides`, then `DomainTag::try_new`; the route reaches no outer-edge collaborator.
- `pairing/factory` entry `a_sampled_scalar_encodes_through_secret`: absorbed — proven by `hash_to_scalar_matches_the_oracle_for_the_reference_tag_and_message` in this module's private integration test element `integration_test.rs`, which samples a uniform `Secret` and encodes the sampled scalar through the received adapter.
- `pairing/factory` entry `a_sampling_refusal_reaches_the_caller_unchanged`: carried — restated in this node's terms: the sampling bound's refusal reaches the caller of `hash_to_scalar` unchanged inside the family's `HashToScalarErrorReturn::Sampling` flavor. Route: `create_pairing`, the arm's concrete through its `try_new`, the consumer, `Keccak256HashToScalar::hash_to_scalar`, then the received scalar's `sample_from_uniform_bytes`; the route reaches no outer-edge collaborator.

## Integration: own entries

Each entry is named and proven by the private integration block of the same name in `integration_test.rs`, its body written once and run over every member of `PAIRING_CONCRETES`.

- `hash_to_scalar_matches_the_oracle_for_the_reference_tag_and_message`: condition `hash_to_scalar` is called through `create_pairing` for a member of `PAIRING_CONCRETES` with `REFERENCE_TAG` and `REFERENCE_MESSAGE`; outcome the encoding of the returned scalar equals the encoding of the scalar the member's `sample_from_uniform_bytes` returns for the `Secret` holding `expected_uniform_bytes(tag, REFERENCE_MESSAGE, S::UNIFORM_BYTES_LENGTH)`; variation every member of the declared set, each with its own scalar type; edge that must survive: the one-byte length prefix, the order tag then message, and the digest zero-extended at the front all reach the sampler exactly as the oracle states them.
- `hash_to_scalar_matches_the_oracle_for_the_empty_message`: condition the same call with `REFERENCE_TAG` and `EMPTY_MESSAGE`; outcome the encoding of the returned scalar equals the encoding of the scalar sampled from `expected_uniform_bytes(tag, EMPTY_MESSAGE, S::UNIFORM_BYTES_LENGTH)`; variation the empty message against the reference message; edge that must survive: the message segment is absent and the digest covers the prefix and the tag alone.
- `hash_to_scalar_binds_the_tag_length`: condition the same call with `BINDING_LONGER_TAG` and `BINDING_SHORTER_MESSAGE`, which concatenate to the same bytes as `BINDING_TAG` and `BINDING_MESSAGE`; outcome the encoding of the returned scalar equals the encoding of the scalar sampled from `expected_uniform_bytes(tag, BINDING_SHORTER_MESSAGE, S::UNIFORM_BYTES_LENGTH)`, whose input begins with the length of the longer tag; variation a split of the same concatenated bytes with the tag one byte longer, so a prefix that is absent or does not equal the given tag's length yields a digest the oracle does not state; edge that must survive: the prefix is the length of the tag given, not a constant.
- `hash_to_scalar_separates_domains`: condition the same call with `OTHER_TAG` and `REFERENCE_MESSAGE`; outcome the encoding of the returned scalar equals the encoding of the scalar sampled from `expected_uniform_bytes(tag, REFERENCE_MESSAGE, S::UNIFORM_BYTES_LENGTH)` for `OTHER_TAG`; variation the reference message under a tag other than `REFERENCE_TAG`; edge that must survive: the tag's bytes, and not only its length, reach the hasher.

## Integration: private surface

The chain `create_pairing`, the arm's concrete through its `try_new`, `DomainTag::try_new` through `build_domain_tag`, `Keccak256HashToScalar::try_new`, `Keccak256HashToScalar::hash_to_scalar` over `sha3`, the received scalar's `sample_from_uniform_bytes`, and the received adapter's `encode_scalar`; outer-edge collaborators mocked: none; the observable result is the encoded scalar bytes read through `Secret::expose`; the entries proven are every own entry and the absorbed entry above.

