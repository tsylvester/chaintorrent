# possession_vectors interactions

`possession_vectors` produces the possession vector records the envelope-key registry's Foundry tests prove against. For each of the configured count of records it draws two uniforms through the randomness family at the length the pairing's scalar sampling bound declares and generates a key pair with its proofs of possession through the key agreement's `generate_keys`, each record carrying the drawn secrets, the public keys, and the proofs' components; the proofs' nonces are drawn inside the adapter and not recorded. The function reads no clock and touches no filesystem; it names no curve, concrete, tag, or count — the count arrives in its params, the proofs' challenges are the key agreement's own under the tags its declaration carries, and every encoding is the selected pairing's.

## Description constructor

`PossessionVectorDescription::try_new(_params: PossessionVectorDescriptionConstructorParams) -> PossessionVectorDescriptionTryNewReturn`: one branch, outcome `Ok(PossessionVectorDescription)`.

## `IEncodingContract for PossessionVectorDescription`

`type Described = PossessionVector`, `type FromFieldsErrorReturn = PossessionVectorFromFieldsErrorReturn`, `FIELDS` `&POSSESSION_VECTOR_FIELD_KINDS`.

`to_fields`:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| mapped | any `PossessionVector` | none | none | `Ok(ToFieldsSuccessReturn { fields: CanonicalFields { values } })`, `values` being, in member order, `CanonicalFieldValue::Unsigned256` of `x`, `CanonicalFieldValue::Unsigned256` of `y`, `CanonicalFieldValue::Bytes` of clones of `pk1`, `pk2`, and `r1`, `CanonicalFieldValue::Unsigned256` of `z1`, `CanonicalFieldValue::Bytes` of a clone of `r2`, and `CanonicalFieldValue::Unsigned256` of `z2` |

`fields_to_value`:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| count | `payload.values` does not convert into `[CanonicalFieldValue; POSSESSION_VECTOR_FIELD_COUNT]` | the conversion | none | `Err(PossessionVectorFromFieldsErrorReturn::FieldCount { expected: POSSESSION_VECTOR_FIELD_COUNT, actual })`, `actual` the vector's length |
| kind | the lowest index `i` whose value is not the kind `POSSESSION_VECTOR_FIELD_KINDS` holds at `i` | the per-index kind check | none | `Err(PossessionVectorFromFieldsErrorReturn::FieldKind { index: i, expected })`, `expected` that kind |
| rebuilt | every field the kind `POSSESSION_VECTOR_FIELD_KINDS` holds at its index | none | none | `Ok(FromFieldsSuccessReturn { described: PossessionVector { x, y, pk1, pk2, r1, z1, r2, z2 } })`, each moved from its field |

## `members`

`members(&self) -> PossessionVectorMembersReturn` on the description: for each index in order, dependency call `SolidityMemberName::try_new(SolidityMemberNameConstructorParams { text })` over `POSSESSION_VECTOR_MEMBER_NAMES` at that index; a refusal returns `Err(PossessionVectorMembersErrorReturn::MemberName(error))` unchanged; each admitted name paired with `POSSESSION_VECTOR_FIELD_KINDS` at the same index as `SolidityRecordMember { name, kind }`; then `SolidityRecordMembers::try_new(SolidityRecordMembersConstructorParams { members })`, a refusal returning `Err(PossessionVectorMembersErrorReturn::RecordMembers(error))` unchanged; otherwise `Ok` of the members. Every name is a distinct lowercase identifier the constructor admits, other than `line`, so no input takes either refusal, and neither has a unit test.

## `possession_vectors`

`possession_vectors<'a, P: IPairingArithmetic, K: IKeyAgreementAdapter<Pairing = P>>(deps: &PossessionVectorsDeps<'a, P, K>, params: PossessionVectorsParams, payload: PossessionVectorsPayload) -> PossessionVectorsReturn`, the trusted form: every input is a typed value from the generate concrete.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| draw failed | `deps.random.fill_bytes` returns `Err(error)` | the fill result | `deps.random.fill_bytes`, once per draw | `Err(PossessionVectorsErrorReturn::FillBytes(error))`, the error unchanged |
| generation failed | `deps.key_agreement.generate_keys` returns `Err(error)` | the generation result | `generate_keys`, once per record | `Err(PossessionVectorsErrorReturn::GenerateKeys(error))`, the error unchanged |
| scalar length | `success.bytes.expose().as_ref()` of an `encode_scalar` result does not convert into `[u8; 32]` | the conversion | `deps.pairing.encode_scalar` | `Err(PossessionVectorsErrorReturn::ScalarLength { actual })`, `actual` the encoding's length |
| computed | every record generated | the records assembled in draw order | the adapter calls stated | `Ok(PossessionVectorsSuccessReturn { records })` |

### Scalar bytes

For any scalar, dependency call `deps.pairing.encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar })`, unpacked irrefutably; condition `<[u8; 32]>::try_from(success.bytes.expose().as_ref())` fails takes the scalar-length branch, `actual` the encoding's length; otherwise the array. Every concrete encodes a scalar to 32 bytes, so no input takes the refusal, and it has no unit test.

### Records

For each index from `0` below `params.count.get()`:

- **Draws.** Two dependency calls `deps.random.fill_bytes(FillBytesParams, FillBytesPayload { length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH })`, held as `x_uniform` then `y_uniform`, a failure taking the draw-failed branch.
- **Generation.** Dependency call `deps.key_agreement.generate_keys(GenerateKeysParams, GenerateKeysPayload { x_uniform, y_uniform })`; a failure takes the generation-failed branch.
- **Components.** Dependency calls `deps.key_agreement.key_pair_components(KeyPairComponentsParams, KeyPairComponentsPayload { key_pair: &success.key_pair })`, `key_pair_public_keys(KeyPairPublicKeysParams, KeyPairPublicKeysPayload { key_pair: &success.key_pair })`, and `possession_components(PossessionComponentsParams, PossessionComponentsPayload { possession: &success.possession })`, each unpacked irrefutably; outcome a `PossessionVector` with `x` and `y` the scalar bytes of clones of `components.x.expose()` and `components.y.expose()`, `pk1` and `r1` the `encode_g1` bytes of `pk1` and `r1`, `pk2` and `r2` the `encode_g2` bytes of `pk2` and `r2`, and `z1` and `z2` the scalar bytes of `z1` and `z2`.

### Ordering

Per record, both draws, the generation, then the components; the first failure returns; every drawn uniform's and every secret component's `Secret` is cleared when it drops; `payload` is not read; no side effect beyond the draws.
