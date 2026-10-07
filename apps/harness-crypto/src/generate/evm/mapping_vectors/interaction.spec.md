# mapping_vectors interactions

`mapping_vectors` produces the hash-to-scalar and identity-mapping vector records the Solidity verifier's Foundry tests prove against. For each of the configured count of messages it draws a message of the configured length and maps it under every domain tag the contracts hash under — the KEM's identity tag, the key agreement's two possession tags, and the delivery proof's challenge and weight tags — emitting one record per tag; for each configured identity it derives the identity element through the credential KEM under the admitted scope and records the element's scalar. The function reads no clock and touches no filesystem; every tag is read from a declaration, every count and length arrives in its params, the scope is the one the KEM factory admitted, and every identity arrives in its payload.

## Description constructors

`HashToScalarVectorDescription::try_new(_params: HashToScalarVectorDescriptionConstructorParams) -> HashToScalarVectorDescriptionTryNewReturn` and `IdentityMappingVectorDescription::try_new(_params: IdentityMappingVectorDescriptionConstructorParams) -> IdentityMappingVectorDescriptionTryNewReturn`: one branch each, outcome `Ok` of the fieldless struct.

## `IEncodingContract for HashToScalarVectorDescription`

`type Described = HashToScalarVector`, `type FromFieldsErrorReturn = HashToScalarVectorFromFieldsErrorReturn`, `FIELDS` `&HASH_TO_SCALAR_VECTOR_FIELD_KINDS`.

`to_fields`:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| mapped | any `HashToScalarVector` | none | none | `Ok(ToFieldsSuccessReturn { fields: CanonicalFields { values } })`, `values` being `CanonicalFieldValue::Bytes` of a clone of `tag`, `CanonicalFieldValue::Bytes` of a clone of `message`, and `CanonicalFieldValue::Unsigned256` of `scalar`, in that order |

`fields_to_value`:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| count | `payload.values` does not convert into `[CanonicalFieldValue; HASH_TO_SCALAR_VECTOR_FIELD_COUNT]` | the conversion | none | `Err(HashToScalarVectorFromFieldsErrorReturn::FieldCount { expected: HASH_TO_SCALAR_VECTOR_FIELD_COUNT, actual })`, `actual` the vector's length |
| kind | the lowest index `i` whose value is not the kind `HASH_TO_SCALAR_VECTOR_FIELD_KINDS` holds at `i` | the per-index kind check | none | `Err(HashToScalarVectorFromFieldsErrorReturn::FieldKind { index: i, expected })`, `expected` that kind |
| rebuilt | every field the kind `HASH_TO_SCALAR_VECTOR_FIELD_KINDS` holds at its index | none | none | `Ok(FromFieldsSuccessReturn { described: HashToScalarVector { tag, message, scalar } })`, each moved from its field |

## `IEncodingContract for IdentityMappingVectorDescription`

`type Described = IdentityMappingVector`, `type FromFieldsErrorReturn = IdentityMappingVectorFromFieldsErrorReturn`, `FIELDS` `&IDENTITY_MAPPING_VECTOR_FIELD_KINDS`.

`to_fields`: one branch, `values` being `CanonicalFieldValue::Bytes` of a clone of `identity` and `CanonicalFieldValue::Unsigned256` of `scalar`, in that order.

`fields_to_value`: the same count, kind, and rebuilt branches over `IDENTITY_MAPPING_VECTOR_FIELD_COUNT` and `IDENTITY_MAPPING_VECTOR_FIELD_KINDS` with `IdentityMappingVectorFromFieldsErrorReturn`, the rebuilt branch's outcome `Ok(FromFieldsSuccessReturn { described: IdentityMappingVector { identity, scalar } })`.

## `members`

`members(&self) -> MappingVectorMembersReturn` on each description: for each index in order, dependency call `SolidityMemberName::try_new(SolidityMemberNameConstructorParams { text })` over the description's member names at that index; a refusal returns `Err(MappingVectorMembersErrorReturn::MemberName(error))` unchanged; each admitted name paired with the description's field kind at the same index as `SolidityRecordMember { name, kind }`; then `SolidityRecordMembers::try_new(SolidityRecordMembersConstructorParams { members })`, a refusal returning `Err(MappingVectorMembersErrorReturn::RecordMembers(error))` unchanged; otherwise `Ok` of the members. Every name is a distinct lowercase identifier the constructor admits, other than `line`, and each record has at least two members, so no input takes either refusal, and neither has a unit test.

## `mapping_vectors`

`mapping_vectors<'a, P: IPairingArithmetic, K: ICredentialKemAdapter<Pairing = P>>(deps: &MappingVectorsDeps<'a, P, K>, params: MappingVectorsParams, payload: MappingVectorsPayload<'_>) -> MappingVectorsReturn`, the trusted form: every input is a typed value from the generate concrete.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| tag refused | `DomainTag::try_new` refuses a declared tag's bytes | the try result | `DomainTag::try_new(DomainTagConstructorParams { bytes })`, once per tag | `Err(MappingVectorsErrorReturn::DomainTag(error))`, the error unchanged |
| draw failed | `deps.random.fill_bytes` returns `Err(error)` | the fill result | `deps.random.fill_bytes`, once per draw | `Err(MappingVectorsErrorReturn::FillBytes(error))`, the error unchanged |
| hash failed | `deps.hash_to_scalar.hash_to_scalar` returns `Err(error)` | the hash result | `hash_to_scalar`, once per record | `Err(MappingVectorsErrorReturn::HashToScalar(error))`, the error unchanged |
| asset identity wrong length | under `IdentityScope::Asset`, `bytes` does not convert into `[u8; ASSET_IDENTITY_HASH_LENGTH]` | the conversion | none | `Err(MappingVectorsErrorReturn::AssetIdentityLength { index: i, actual })`, `actual` the bytes' length |
| asset identity refused | `AssetIdentityHash::try_new` refuses the converted bytes | the try result | `AssetIdentityHash::try_new`, once per asset identity | `Err(MappingVectorsErrorReturn::AssetIdentityHash(error))`, the error unchanged |
| setup failed | `deps.kem.setup` returns `Err(error)` | the setup result | `deps.kem.setup`, once per identity | `Err(MappingVectorsErrorReturn::Setup(error))`, the error unchanged |
| derivation failed | `deps.kem.derive_identity` returns `Err(error)` | the derivation result | `deps.kem.derive_identity`, once per identity | `Err(MappingVectorsErrorReturn::DeriveIdentity(error))`, the error unchanged |
| scalar length | `success.bytes.expose().as_ref()` of an `encode_scalar` result does not convert into `[u8; 32]` | the conversion | `deps.pairing.encode_scalar` | `Err(MappingVectorsErrorReturn::ScalarLength { actual })`, `actual` the encoding's length |
| computed | every tag admitted and every record computed | the records assembled in the order below | the adapter calls stated | `Ok(MappingVectorsSuccessReturn { hash_to_scalar, identity_mapping })` |

### Tags

Dependency call `DomainTag::try_new(DomainTagConstructorParams { bytes })` over a copy of each of `K::DECLARATION.identity_tag`, `payload.key_agreement.possession_g1_tag`, `payload.key_agreement.possession_g2_tag`, `payload.delivery_proof.challenge_tag`, and `payload.delivery_proof.weight_tag`, in that order; a refusal takes the tag-refused branch. Every declared tag is printable ASCII with no edge space, so no input takes the refusal, and it has no unit test.

### Scalar bytes

For any scalar, dependency call `deps.pairing.encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar })`, unpacked irrefutably; condition `<[u8; 32]>::try_from(success.bytes.expose().as_ref())` fails takes the scalar-length branch, `actual` the encoding's length; otherwise the array. Every concrete encodes a scalar to 32 bytes, so no input takes the refusal, and it has no unit test.

### Hash-to-scalar records

For each index from `0` below `params.hash_to_scalar_count.get()`: dependency call `deps.random.fill_bytes(FillBytesParams, FillBytesPayload { length: params.message_length })`, a failure taking the draw-failed branch, the message being a copy of `success.bytes.expose()`; then for each tag in tag order, dependency call `deps.hash_to_scalar.hash_to_scalar(HashToScalarParams { tag: &tag }, HashToScalarPayload { message: &message })`, a failure taking the hash-failed branch; outcome a `HashToScalarVector` with `tag` the tag's declared bytes, `message` a clone of the message, and `scalar` the scalar's bytes.

### Setup draws

Three dependency calls `deps.random.fill_bytes(FillBytesParams, FillBytesPayload { length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH })`, each failure taking the draw-failed branch, held as `master_uniform`, `u0_uniform`, and `u1_uniform`. The draws do not reach the record — the identity element's scalar is a function of the tag and the bytes alone.

### Identity-mapping records

For each index `i` and bytes of `payload.identities` in order, decision an exhaustive `match` on `params.scope`:

- `IdentityScope::Entitlement`: the setup draws; dependency call `deps.kem.setup(SetupParams { scope: SetupScope::Entitlement }, SetupPayload { master_uniform, u0_uniform, u1_uniform })`, a failure taking the setup-failed branch; then `deps.kem.derive_identity(DeriveIdentityParams, DeriveIdentityPayload { parameter_set: &success.parameter_set, identity: KemIdentity::Entitlement { canonical: bytes } })`, a failure taking the derivation-failed branch.
- `IdentityScope::Asset`: condition `<[u8; ASSET_IDENTITY_HASH_LENGTH]>::try_from(bytes.as_slice())` fails takes the asset-identity-wrong-length branch; then `AssetIdentityHash::try_new(AssetIdentityHashConstructorParams { bytes })`, a refusal taking the asset-identity-refused branch; the setup draws; `deps.kem.setup(SetupParams { scope: SetupScope::Asset { identity_hash: &hash } }, SetupPayload { master_uniform, u0_uniform, u1_uniform })` and `deps.kem.derive_identity(DeriveIdentityParams, DeriveIdentityPayload { parameter_set: &success.parameter_set, identity: KemIdentity::Asset { identity_hash: &hash } })`, each failure taking the setup-failed or derivation-failed branch as in the entitlement arm.

Then dependency call `deps.kem.identity_element_components(IdentityElementComponentsParams, IdentityElementComponentsPayload { identity_element: &success.identity_element })`, unpacked irrefutably; outcome an `IdentityMappingVector` with `identity` a clone of the bytes and `scalar` the bytes of `components.scalar`.

### Ordering

The tags, then the hash-to-scalar records message by message, then the identity-mapping records in payload order, the first refusal returned; every drawn uniform's `Secret` is cleared when it drops; no side effect beyond the draws.
