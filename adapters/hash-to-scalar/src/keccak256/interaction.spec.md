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
| sampling input too short | the prefix conversion passes and `S::UNIFORM_BYTES_LENGTH.checked_sub(KECCAK256_DIGEST_LENGTH)` is `None` | the subtraction, before hashing | none | `Err(HashToScalarErrorReturn::Keccak256(Keccak256HashToScalarErrorReturn::UniformLengthBelowDigest { uniform_length: S::UNIFORM_BYTES_LENGTH, digest_length: KECCAK256_DIGEST_LENGTH }))`; every current scalar type samples from 64 bytes, so no input takes this branch and it has no unit test |
| sampling refused | both checks pass and `S::sample_from_uniform_bytes` returns `Err(error)` | the sampler's result | the hashing calls below, then `S::sample_from_uniform_bytes` once | `Err(HashToScalarErrorReturn::Keccak256(Keccak256HashToScalarErrorReturn::Sampling(error)))`, the error unchanged; the input is exactly `S::UNIFORM_BYTES_LENGTH` bytes and the sampler's one refusal is a wrong length, so no input takes this branch and it has no unit test |
| hashed | both checks pass and sampling succeeds | all checks pass | `Keccak256::new()`, then `update` with the one prefix byte, `update` with `params.tag.as_bytes()`, and `update` with `payload.message`, in that order, each once, then `finalize` once; a buffer of `S::UNIFORM_BYTES_LENGTH` zero bytes receives the digest in its last `KECCAK256_DIGEST_LENGTH` bytes and is moved into a `Secret` by `let Ok(uniform) = Secret::try_new(SecretConstructorParams { value: buffer });`; then `S::sample_from_uniform_bytes(SampleUniformScalarParams, SampleUniformScalarPayload { uniform })` once | `Ok(HashToScalarSuccessReturn { scalar })`, `scalar` a clone of the sampled `Secret`'s exposed value, the `Secret` then dropping and zeroizing its copy |

## Ordering and invariants

- The prefix conversion and the length subtraction precede the hasher; the prefix, the tag, and the message are absorbed in that order. The same tag and message always yield the same scalar for one scalar type.
- The digest is read as a big-endian integer, zero-extended at the front to `S::UNIFORM_BYTES_LENGTH`, and reduced modulo the group order by the scalar type's own sampler — no field arithmetic is duplicated here, and no pairing library or use's tag is named.

