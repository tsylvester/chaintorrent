# `mapping_vectors` — interaction spec

Branch contract for the `mapping_vectors` module of the `solidity` module of the `generate` module of `apps/harness-crypto`: `mapping_vectors` emits the hash-to-scalar vector records, a drawn kind, each drawn message mapped under every domain tag the constants library carries, and the identity-mapping vector records, a kind over configured inputs, each admitted identity mapped through the credential KEM under its own setup scope and read from the identity element's components, beside the two record descriptions and their member names. Each function's branches state condition, decision, dependency call, and the exact return outcome.

## `HashToScalarVectorDescription::try_new`

`try_new(_params: HashToScalarVectorDescriptionConstructorParams) -> HashToScalarVectorDescriptionTryNewReturn`: one branch; outcome `Ok(HashToScalarVectorDescription)`.

## `IdentityMappingVectorDescription::try_new`

`try_new(_params: IdentityMappingVectorDescriptionConstructorParams) -> IdentityMappingVectorDescriptionTryNewReturn`: one branch; outcome `Ok(IdentityMappingVectorDescription)`.

## `IEncodingContract` for `HashToScalarVectorDescription`

`type Described = HashToScalarVector`, `type FromFieldsErrorReturn = HashToScalarVectorFromFieldsErrorReturn`, `FIELDS` `&HASH_TO_SCALAR_VECTOR_FIELD_KINDS`.

`to_fields(params: ToFieldsParams, payload: &HashToScalarVector) -> ToFieldsReturn`: one branch; outcome `Ok(ToFieldsSuccessReturn { fields: CanonicalFields { values } })`, `values` being `CanonicalFieldValue::Bytes` of a clone of `tag`, `CanonicalFieldValue::Bytes` of a clone of `message`, and `CanonicalFieldValue::Unsigned256` of `scalar`, in that order.

`fields_to_value(params: FromFieldsParams, payload: CanonicalFields) -> FromFieldsReturn<HashToScalarVector, HashToScalarVectorFromFieldsErrorReturn>`:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| fields refused | `CanonicalFieldReader::try_new`, a `read_bytes`, or the `read_unsigned256` returns `Err(error)` | the call's return | `CanonicalFieldReader::try_new(payload, Self::FIELDS)`, then `read_bytes` for `tag`, `read_bytes` for `message`, and `read_unsigned256` for `scalar`, in that order | `Err(HashToScalarVectorFromFieldsErrorReturn::Fields(error))`, the refusal unchanged |
| rebuilt | the reader's construction and every read return `Ok` | none | the same calls | `Ok(FromFieldsSuccessReturn { described: HashToScalarVector { tag, message, scalar } })`, each the value its read returned |

## `IEncodingContract` for `IdentityMappingVectorDescription`

`type Described = IdentityMappingVector`, `type FromFieldsErrorReturn = IdentityMappingVectorFromFieldsErrorReturn`, `FIELDS` `&IDENTITY_MAPPING_VECTOR_FIELD_KINDS`.

`to_fields(params: ToFieldsParams, payload: &IdentityMappingVector) -> ToFieldsReturn`: one branch; outcome `Ok(ToFieldsSuccessReturn { fields: CanonicalFields { values } })`, `values` being `CanonicalFieldValue::Bytes` of a clone of `identity` and `CanonicalFieldValue::Unsigned256` of `scalar`, in that order.

`fields_to_value(params: FromFieldsParams, payload: CanonicalFields) -> FromFieldsReturn<IdentityMappingVector, IdentityMappingVectorFromFieldsErrorReturn>`:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| fields refused | `CanonicalFieldReader::try_new`, a `read_bytes`, or the `read_unsigned256` returns `Err(error)` | the call's return | `CanonicalFieldReader::try_new(payload, Self::FIELDS)`, then `read_bytes` for `identity` and `read_unsigned256` for `scalar`, in that order | `Err(IdentityMappingVectorFromFieldsErrorReturn::Fields(error))`, the refusal unchanged |
| rebuilt | the reader's construction and every read return `Ok` | none | the same calls | `Ok(FromFieldsSuccessReturn { described: IdentityMappingVector { identity, scalar } })`, each the value its read returned |

## `member_names`

`member_names(&self) -> MappingVectorMemberNamesReturn` on each description: for each index in order, dependency call `SolidityMemberName::try_new(SolidityMemberNameConstructorParams { text })` over the description's member names at that index — `HASH_TO_SCALAR_VECTOR_MEMBER_NAMES` on the hash-to-scalar description, `IDENTITY_MAPPING_VECTOR_MEMBER_NAMES` on the identity-mapping description.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| refused | `SolidityMemberName::try_new` returns `Err(error)` at an index | the constructor's return | the constructor, once per name in order | `Err(MappingVectorMemberNamesErrorReturn::MemberName(error))`, the refusal unchanged |
| admitted | every name is admitted | none | the constructor, once per name | `Ok` of the admitted names in order |

Every name is a lowercase identifier the constructor admits, other than `line`, so no input takes the refusal.

## `mapping_vectors`

`mapping_vectors<'a, P: IPairingArithmetic, K: ICredentialKemAdapter<Pairing = P>>(deps: &MappingVectorsDeps<'a, P, K>, params: MappingVectorsParams, payload: MappingVectorsPayload<'_>) -> MappingVectorsReturn`, the trusted form: every input is a typed value from the generate concrete. The function reads no clock and touches no filesystem; it names no curve, concrete, tag, count, length, scope, or identity, and admits no identity; every tag is read from a declaration, every count and length arrives in its params, and every identity arrives in its payload admitted, paired with its setup scope.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| tag refused | `DomainTag::try_new` returns `Err(error)` on a declared tag | the constructor's return | `DomainTag::try_new(DomainTagConstructorParams { bytes })`, once per declared tag in order, before any draw | `Err(MappingVectorsErrorReturn::DomainTag(error))`, the refusal unchanged |
| fill refused | `deps.random.fill_bytes` returns `Err(error)` on a draw | the call's return | `deps.random.fill_bytes(FillBytesParams, FillBytesPayload { length })` | `Err(MappingVectorsErrorReturn::FillBytes(error))`, the error unchanged |
| hash refused | `deps.hash_to_scalar.hash_to_scalar` returns `Err(error)` on a record | the call's return | `deps.hash_to_scalar.hash_to_scalar(HashToScalarParams { tag: &tag }, HashToScalarPayload { message: &message })` | `Err(MappingVectorsErrorReturn::HashToScalar(error))`, the error unchanged |
| setup refused | `deps.kem.setup` returns `Err(error)` on an identity | the call's return | `deps.kem.setup(SetupParams { scope: identity.setup_scope }, SetupPayload { master_uniform, u0_uniform, u1_uniform })` | `Err(MappingVectorsErrorReturn::Setup(error))`, the error unchanged |
| derivation refused | `deps.kem.derive_identity` returns `Err(error)` on an identity | the call's return | `deps.kem.derive_identity(DeriveIdentityParams, DeriveIdentityPayload { parameter_set: &success.parameter_set, identity: identity.identity })` | `Err(MappingVectorsErrorReturn::DeriveIdentity(error))`, the error unchanged |
| scalar length | `<[u8; 32]>::try_from(success.bytes.expose().as_ref())` fails on an encoded scalar | the conversion's return | `deps.pairing.encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar })` | `Err(MappingVectorsErrorReturn::ScalarLength { actual })`, `actual` the encoding's length |
| computed | every tag admits and every draw, hash, setup, and derivation succeeds | none | the calls each subsection states | `Ok(MappingVectorsSuccessReturn { hash_to_scalar, identity_mapping })` |

### The tags

Dependency call `DomainTag::try_new(DomainTagConstructorParams { bytes })` over a copy of each of `K::DECLARATION.identity_tag`, `payload.key_agreement.possession_g1_tag`, `payload.key_agreement.possession_g2_tag`, `payload.delivery_proof.challenge_tag`, and `payload.delivery_proof.weight_tag`, in that order; a refusal returns `Err(MappingVectorsErrorReturn::DomainTag(error))` unchanged, before any draw.

### Scalar bytes

For any scalar, dependency call `deps.pairing.encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar })`, unpacked irrefutably; condition `<[u8; 32]>::try_from(success.bytes.expose().as_ref())` fails, outcome `Err(MappingVectorsErrorReturn::ScalarLength { actual })`, `actual` the encoding's length; otherwise the array.

### Hash-to-scalar records

For each index from `0` below `params.hash_to_scalar_count.get()`: dependency call `deps.random.fill_bytes(FillBytesParams, FillBytesPayload { length: params.message_length })`, a failure returning `Err(MappingVectorsErrorReturn::FillBytes(error))` unchanged, the message being a copy of `success.bytes.expose()`; then for each tag in tag order, dependency call `deps.hash_to_scalar.hash_to_scalar(HashToScalarParams { tag: &tag }, HashToScalarPayload { message: &message })`, a failure returning `Err(MappingVectorsErrorReturn::HashToScalar(error))` unchanged; outcome a `HashToScalarVector` with `tag` the tag's declared bytes, `message` a clone of the message, and `scalar` the scalar's bytes.

### Setup draws

Three dependency calls `deps.random.fill_bytes(FillBytesParams, FillBytesPayload { length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH })`, each failure returning `Err(MappingVectorsErrorReturn::FillBytes(error))` unchanged, held as `master_uniform`, `u0_uniform`, and `u1_uniform`.

### Identity-mapping records

For each admitted identity `identity` of `payload.identities` in order: the setup draws; dependency call `deps.kem.setup(SetupParams { scope: identity.setup_scope }, SetupPayload { master_uniform, u0_uniform, u1_uniform })`, a failure returning `Err(MappingVectorsErrorReturn::Setup(error))` unchanged; then `deps.kem.derive_identity(DeriveIdentityParams, DeriveIdentityPayload { parameter_set: &success.parameter_set, identity: identity.identity })`, a failure returning `Err(MappingVectorsErrorReturn::DeriveIdentity(error))` unchanged.

### Record bytes

Decision an exhaustive `match` on `identity.identity`; `KemIdentity::Entitlement { canonical }`, outcome a copy of `canonical`; `KemIdentity::Asset { root }`, outcome a copy of `root.as_bytes()`.

### The identity element

Dependency call `deps.kem.identity_element_components(IdentityElementComponentsParams, IdentityElementComponentsPayload { identity_element: &success.identity_element })`, unpacked irrefutably; outcome an `IdentityMappingVector` with `identity` the record bytes and `scalar` the bytes of `components.scalar`.

### Ordering

The tags, then the hash-to-scalar records message by message, then the identity-mapping records in payload order, the first refusal returned; every drawn uniform's `Secret` is cleared when it drops; no side effect beyond the draws.

## Integration: callee dispositions

- `every_concrete_declaring_the_keccak256_identifier_hashes_the_reference_inputs_to_the_oracle` (`hash-to-scalar/factory`): absorbed, proven by `each_hash_to_scalar_record_is_the_references_mapping_of_its_message_under_its_tag` in the private integration test element `integration_test.rs`; restated in this node's terms as: the scalar of each hash-to-scalar record encodes the scalar the received scalar type samples from `expected_uniform_bytes` over the tag built from the record's `tag`, the record's `message`, and `S::UNIFORM_BYTES_LENGTH`.
- `an admitted tag reads back through the crate root` (`domain_tag`, recorded by `hash-to-scalar/factory`): absorbed, proven by `each_hash_to_scalar_record_is_the_references_mapping_of_its_message_under_its_tag`; restated in this node's terms as: the tag each record is hashed under is the tag whose bytes are the record's `tag`, which are the bytes the declaration states for its position.
- `every_concrete_maps_an_identity_under_its_declared_tag` (`kem/factory`): absorbed, proven by `each_identity_record_is_the_references_mapping_of_its_identity_under_the_identity_tag`; restated in this node's terms as: the scalar of each identity-mapping record encodes the scalar the received scalar type samples from `expected_uniform_bytes` over the tag built from `K::DECLARATION.identity_tag`, the record's `identity`, and `S::UNIFORM_BYTES_LENGTH`.
- `every_concrete_derives_under_the_asset_scope_only_the_identity_the_set_fixes` (`kem/factory`): absorbed, proven by `each_identity_record_is_the_references_mapping_of_its_identity_under_the_identity_tag`; restated in this node's terms as: under the asset scope, `derive_identity` over the hash the admitted identity's setup scope names returns `Ok`, so every asset identity admitted with its own setup scope yields its record.
- `a_sampled_scalar_encodes_through_secret` (`pairing/factory`): absorbed, proven by `each_hash_to_scalar_record_is_the_references_mapping_of_its_message_under_its_tag` and `each_identity_record_is_the_references_mapping_of_its_identity_under_the_identity_tag`; restated in this node's terms as: the scalar a record holds is `encode_scalar`'s bytes of the scalar returned, read through `Secret::expose`, equal to the encoding of the scalar sampled from the reference's uniform bytes.
- `every_concrete_returns_the_sampling_bound_refusal_whole` (`hash-to-scalar/factory`): carried; restated in this node's terms as: the sampling bound's refusal reaches the caller of `mapping_vectors` whole as `Err(MappingVectorsErrorReturn::HashToScalar(error))`; route `create_pairing`, the arm's concrete through its `try_new`, `create_hash_to_scalar` selecting the mock concrete under `MockIHashToScalarAdapterFailureMode::SamplingRefused`, the adapter's `hash_to_scalar`, then `mapping_vectors` at its hash-to-scalar step; the outer-edge collaborator on the route is the hash-to-scalar concrete, the family's mock concrete under its failure mode.
- `every_concrete_returns_a_hash_to_scalar_refusal_whole` (`kem/factory`): carried; restated in this node's terms as: the refusal reaches the caller of `mapping_vectors` whole as `Err(MappingVectorsErrorReturn::Setup(SetupErrorReturn::HashToScalar(error)))` under the asset scope and as `Err(MappingVectorsErrorReturn::DeriveIdentity(DeriveIdentityErrorReturn::HashToScalar(error)))` under the entitlement scope; route `create_pairing`, the arm's concrete through its `try_new`, `create_hash_to_scalar` selecting the mock concrete under `MockIHashToScalarAdapterFailureMode::SamplingRefused`, `create_kem`, the selected KEM concrete's `try_new`, then `setup` or `derive_identity` inside `mapping_vectors`; the outer-edge collaborator on the route is the hash-to-scalar concrete, the family's mock concrete under its failure mode.
- `a refusal reaches the caller whole and yields no tag` (`domain_tag`, recorded by `hash-to-scalar/factory`): carried; restated in this node's terms as: the refusal of a declared tag reaches the caller of `mapping_vectors` whole as `Err(MappingVectorsErrorReturn::DomainTag(error))`, with no draw and no hash made; route `DomainTag::try_new`, then `mapping_vectors` at its tags step; the route reaches no outer-edge collaborator.

## Integration: own entries

Each entry is proven by the block of the private integration test element `integration_test.rs` of the same name.

- `each_hash_to_scalar_record_is_the_references_mapping_of_its_message_under_its_tag`: condition `mapping_vectors` is called with a `hash_to_scalar_count` above one, declared tags that differ from one another, and the operating-system source; outcome each hash-to-scalar record's `tag` equals the bytes the declaration states for its position, `K::DECLARATION.identity_tag` first, and its `scalar` equals the encoding, through `encode_scalar`, of the scalar the received scalar type samples from `expected_uniform_bytes` over the tag built from the record's `tag`, the record's `message`, and `S::UNIFORM_BYTES_LENGTH`; variation every member of `PAIRING_CONCRETES` over each case of `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declares `HashToScalarIdentifier::Keccak256V1` and each case of `DECLARED_KEM_SELECTIONS`, so a hash under another tag, or over another message than the one the record holds, yields a scalar the oracle does not state; edge that must survive the composition: the reference is built through `expected_uniform_bytes`, `sample_from_uniform_bytes`, and `encode_scalar`, a path other than the function's.
- `each_identity_record_is_the_references_mapping_of_its_identity_under_the_identity_tag`: condition `mapping_vectors` is called under the scope of a case of `DECLARED_KEM_SELECTIONS`, with two admitted identities that differ, each paired with that scope, entitlement bytes under the entitlement scope and hashes of distinct 32-byte values under the asset scope; outcome each identity-mapping record's `identity` equals its identity's bytes in payload order and its `scalar` equals the encoding of the scalar the received scalar type samples from `expected_uniform_bytes` over the tag built from `K::DECLARATION.identity_tag`, the record's `identity`, and `S::UNIFORM_BYTES_LENGTH`; variation every member of `PAIRING_CONCRETES` over each case of `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declares `HashToScalarIdentifier::Keccak256V1`, each scope the declared KEM set holds, and operating-system setup draws that differ per identity, so a scalar swapped between records, or one that depends on a setup draw, fails; edge that must survive the composition: the scalar is read from the identity element the real KEM derives, so it is the mapping the issuer and the contract both compute.
- `a_record_survives_its_description_round_trip`: condition each description's `fields_to_value` is called with the fields its `to_fields` returns for a record whose fields differ from one another in value and length; outcome `Ok` whose `described` equals the record; variation both descriptions, whose fields differ in kind and count, so a swapped, dropped, or retyped field fails; edge that must survive the composition: the scalar returns as the same 32-byte array.
- `member_names_declare_one_name_per_field`: condition `member_names()` is called on each description; outcome `Ok` of the declared names in order, under the names of the description's record, and their count equals the length of the description's `FIELDS`; variation both descriptions, whose member counts differ, so a names list that does not follow the fields fails; edge that must survive the composition: the names are admitted by the real `SolidityMemberName::try_new`.

## Integration: private surface

The chain of real functions `run_mapping_vectors_case`, `create_pairing`, the arm's concrete through its `try_new`, `create_hash_to_scalar`, the selected concrete's `try_new` and `hash_to_scalar`, `create_kem`, the selected KEM concrete's `try_new`, `setup`, `derive_identity`, and `identity_element_components`, `create_random_source` for `RandomSourceKind::OperatingSystem`, `DomainTag::try_new`, `mapping_vectors`, each description's `to_fields`, `fields_to_value`, and `member_names()`, and `SolidityMemberName::try_new`; the cases are every member of `PAIRING_CONCRETES` over `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declare `HashToScalarIdentifier::Keccak256V1` and `DECLARED_KEM_SELECTIONS`; no outer-edge collaborator is mocked, the operating system's generator behind `create_random_source` running real and every value an entry compares being read back from the values returned; the observable result is the `MappingVectorsReturn`, the encoded scalars the entries compare through `encode_scalar`, the fields, the rebuilt record, and the names; the entries proven are the own entries above and the absorbed entries.
