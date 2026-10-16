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

## Integration: callee dispositions

The entries of `pairing/factory`, `hash-to-scalar/factory`, `hash-to-scalar/domain_tag`, and `kem/factory` that this node's routes reach, each dispositioned as stated.

- `every_concrete_declaring_the_keccak256_identifier_hashes_the_reference_inputs_to_the_oracle` (`hash-to-scalar/factory`): absorbed — proven by `each_hash_to_scalar_record_is_the_references_mapping_of_its_message_under_its_tag` in the private integration test element `integration_test.rs`; restated in this node's terms: the scalar of each hash-to-scalar record encodes the scalar the received scalar type samples from `expected_uniform_bytes` over the tag built from the record's `tag`, the record's `message`, and `S::UNIFORM_BYTES_LENGTH`.
- `an_admitted_tag_reads_back_through_the_crate_root` (`hash-to-scalar/domain_tag`, recorded by `hash-to-scalar/factory`): absorbed — proven by `each_hash_to_scalar_record_is_the_references_mapping_of_its_message_under_its_tag`; restated in this node's terms: the tag each record is hashed under is the tag whose bytes are the record's `tag`, which are the bytes the declaration states for its position.
- `every_concrete_maps_an_identity_under_its_declared_tag` (`kem/factory`): absorbed — proven by `each_identity_record_is_the_references_mapping_of_its_identity_under_the_identity_tag`; restated in this node's terms: the scalar of each identity-mapping record encodes the scalar the received scalar type samples from `expected_uniform_bytes` over the tag built from `K::DECLARATION.identity_tag`, the record's `identity`, and `S::UNIFORM_BYTES_LENGTH`.
- `every_concrete_derives_under_the_asset_scope_only_the_identity_the_set_fixes` (`kem/factory`): absorbed — proven by `each_identity_record_is_the_references_mapping_of_its_identity_under_the_identity_tag`; restated in this node's terms: under the asset scope, `derive_identity` over the hash the parameter set was set up for returns `Ok`, so every asset identity yields its record.
- `a_sampled_scalar_encodes_through_secret` (`pairing/factory`): absorbed — proven by `each_hash_to_scalar_record_is_the_references_mapping_of_its_message_under_its_tag` and `each_identity_record_is_the_references_mapping_of_its_identity_under_the_identity_tag`; restated in this node's terms: the scalar a record holds is `encode_scalar`'s bytes of the scalar returned, read through `Secret::expose`, equal to the encoding of the scalar sampled from the reference's uniform bytes.
- `every_concrete_returns_the_sampling_bound_refusal_whole` (`hash-to-scalar/factory`): carried — restated in this node's terms: the sampling bound's refusal reaches the caller of `mapping_vectors` whole as `Err(MappingVectorsErrorReturn::HashToScalar(error))`. Route: `create_pairing`, the arm's concrete through its `try_new`, `create_hash_to_scalar` selecting the mock concrete under `MockIHashToScalarAdapterFailureMode::SamplingRefused`, the adapter's `hash_to_scalar`, then `mapping_vectors` at its hash-to-scalar step; the outer-edge collaborator on the route is the hash-to-scalar concrete, the family's mock concrete under its failure mode.
- `every_concrete_returns_a_hash_to_scalar_refusal_whole` (`kem/factory`): carried — restated in this node's terms: the refusal reaches the caller of `mapping_vectors` whole as `Err(MappingVectorsErrorReturn::Setup(SetupErrorReturn::HashToScalar(error)))` under the asset scope and as `Err(MappingVectorsErrorReturn::DeriveIdentity(DeriveIdentityErrorReturn::HashToScalar(error)))` under the entitlement scope. Route: `create_pairing`, the arm's concrete through its `try_new`, `create_hash_to_scalar` selecting the mock concrete under `MockIHashToScalarAdapterFailureMode::SamplingRefused`, `create_kem`, the selected KEM concrete's `try_new`, then `setup` or `derive_identity` inside `mapping_vectors`; the outer-edge collaborator on the route is the hash-to-scalar concrete, the family's mock concrete under its failure mode.
- `a_refusal_reaches_the_caller_whole_and_yields_no_tag` (`hash-to-scalar/domain_tag`, recorded by `hash-to-scalar/factory`): carried — restated in this node's terms: the refusal of a declared tag reaches the caller of `mapping_vectors` whole as `Err(MappingVectorsErrorReturn::DomainTag(error))`, with no draw and no hash made. Route: `DomainTag::try_new`, then `mapping_vectors` at its tags step; the route reaches no outer-edge collaborator.

## Integration: own entries

Each entry is named and proven by the private integration block of the same name in `integration_test.rs`.

- `each_hash_to_scalar_record_is_the_references_mapping_of_its_message_under_its_tag`: condition `mapping_vectors` is called with a `hash_to_scalar_count` above one, declared tags that differ from one another, and the operating-system source; outcome each hash-to-scalar record's `tag` equals the bytes the declaration states for its position, `K::DECLARATION.identity_tag` first, and its `scalar` equals the encoding, through `encode_scalar`, of the scalar the received scalar type samples from `expected_uniform_bytes` over the tag built from the record's `tag`, the record's `message`, and `S::UNIFORM_BYTES_LENGTH`; variation every member of `PAIRING_CONCRETES` over each case of `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declares `HashToScalarIdentifier::Keccak256V1` and each case of `DECLARED_KEM_SELECTIONS`, so a hash under another tag, or over another message than the one the record holds, yields a scalar the oracle does not state; edge that must survive the composition: the reference is built through `expected_uniform_bytes`, `sample_from_uniform_bytes`, and `encode_scalar`, a path other than the function's.
- `each_identity_record_is_the_references_mapping_of_its_identity_under_the_identity_tag`: condition `mapping_vectors` is called under the scope of a case of `DECLARED_KEM_SELECTIONS`, with two identities that differ, entitlement bytes under the entitlement scope and 32-byte values under the asset scope; outcome each identity-mapping record's `identity` equals its configured bytes in payload order and its `scalar` equals the encoding of the scalar the received scalar type samples from `expected_uniform_bytes` over the tag built from `K::DECLARATION.identity_tag`, the record's `identity`, and `S::UNIFORM_BYTES_LENGTH`; variation every member of `PAIRING_CONCRETES` over each case of `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declares `HashToScalarIdentifier::Keccak256V1`, each scope the declared KEM set holds, and operating-system setup draws that differ per identity, so a scalar swapped between records, or one that depends on a setup draw, fails; edge that must survive the composition: the scalar is read from the identity element the real KEM derives, so it is the mapping the issuer and the contract both compute.
- `a_record_survives_its_description_round_trip`: condition each description's `fields_to_value` is called with the fields its `to_fields` returns for a record whose fields differ from one another in value and length; outcome `Ok` whose `described` equals the record; variation both descriptions, whose fields differ in kind and count, so a swapped, dropped, or retyped field fails; edge that must survive the composition: the scalar returns as the same 32-byte array.
- `members_declare_the_kinds_to_fields_emits`: condition `members()` and `to_fields` are called on each description over a built record; outcome the members' kinds in order are the kinds of the values `to_fields` returns, `Bytes` for a `CanonicalFieldValue::Bytes` and `Unsigned256` for a `CanonicalFieldValue::Unsigned256`, under the names of the description's record; variation both descriptions, whose member counts differ, so a members list that does not follow the fields fails; edge that must survive the composition: the names and members are admitted by the real `SolidityMemberName::try_new` and `SolidityRecordMembers::try_new`.

## Integration: private surface

The chain of real functions `create_pairing`, the arm's concrete through its `try_new`, `create_hash_to_scalar`, the selected concrete's `try_new` and `hash_to_scalar`, `create_kem`, the selected KEM concrete's `try_new`, `setup`, `derive_identity`, and `identity_element_components`, `create_random_source` for `RandomSourceKind::OperatingSystem`, `DomainTag::try_new`, `AssetIdentityHash::try_new`, `mapping_vectors`, each description's `to_fields`, `fields_to_value`, and `members()`, `SolidityMemberName::try_new`, and `SolidityRecordMembers::try_new`; the cases are every member of `PAIRING_CONCRETES` over `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declare `HashToScalarIdentifier::Keccak256V1` and `DECLARED_KEM_SELECTIONS`; no outer-edge collaborator is mocked, the operating system's generator behind `create_random_source` running real and every value an entry compares being read back from the values returned; the observable result is the `MappingVectorsReturn`, the encoded scalars the entries compare through `encode_scalar`, the fields, the rebuilt record, and the members; the entries proven are the own entries above and the absorbed entries.
