# `possession_vectors` — interaction spec

Branch contract for the `possession_vectors` module of the `solidity` module of the `generate` module of `apps/harness-crypto`: `possession_vectors` emits the possession vector records, a drawn kind — envelope key pairs generated through the key-agreement family from secret draws the function makes, each record carrying the drawn secrets, the public keys, and the proofs of possession's components, the proofs' nonces drawn inside the adapter and not recorded — beside the record's description and its member names. Each function's branches state condition, decision, dependency call, and the exact return outcome.

## `PossessionVectorDescription::try_new`

`try_new(_params: PossessionVectorDescriptionConstructorParams) -> PossessionVectorDescriptionTryNewReturn`: one branch; outcome `Ok(PossessionVectorDescription)`.

## `IEncodingContract` for `PossessionVectorDescription`

`type Described = PossessionVector`, `type FromFieldsErrorReturn = PossessionVectorFromFieldsErrorReturn`, `FIELDS` `&POSSESSION_VECTOR_FIELD_KINDS`.

`to_fields(params: ToFieldsParams, payload: &PossessionVector) -> ToFieldsReturn`: one branch; outcome `Ok(ToFieldsSuccessReturn { fields: CanonicalFields { values } })`, `values` being, in member order, `CanonicalFieldValue::Unsigned256` of `x`, `CanonicalFieldValue::Unsigned256` of `y`, `CanonicalFieldValue::Bytes` of a clone of `pk1`, `CanonicalFieldValue::Bytes` of a clone of `pk2`, `CanonicalFieldValue::Bytes` of a clone of `r1`, `CanonicalFieldValue::Unsigned256` of `z1`, `CanonicalFieldValue::Bytes` of a clone of `r2`, and `CanonicalFieldValue::Unsigned256` of `z2`.

`fields_to_value(params: FromFieldsParams, payload: CanonicalFields) -> FromFieldsReturn<PossessionVector, PossessionVectorFromFieldsErrorReturn>`:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| count | `payload.values` does not convert into `[CanonicalFieldValue; POSSESSION_VECTOR_FIELD_COUNT]` | the conversion's return | none | `Err(PossessionVectorFromFieldsErrorReturn::FieldCount { expected: POSSESSION_VECTOR_FIELD_COUNT, actual })`, `actual` the vector's length |
| kind | the lowest index `i` whose value is not the kind `POSSESSION_VECTOR_FIELD_KINDS` holds at `i` | that index's kind against the kinds constant | none | `Err(PossessionVectorFromFieldsErrorReturn::FieldKind { index: i, expected })`, `expected` the kind `POSSESSION_VECTOR_FIELD_KINDS` holds at `i` |
| rebuilt | the count converts and every value holds the kind its index declares | none | none | `Ok(FromFieldsSuccessReturn { described: PossessionVector { x, y, pk1, pk2, r1, z1, r2, z2 } })`, each moved from its field |

## `member_names`

`member_names(&self) -> PossessionVectorMemberNamesReturn`: for each index in order, dependency call `SolidityMemberName::try_new(SolidityMemberNameConstructorParams { text })` over `POSSESSION_VECTOR_MEMBER_NAMES` at that index.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| refused | `SolidityMemberName::try_new` returns `Err(error)` at an index | the constructor's return | the constructor, once per name in order | `Err(PossessionVectorMemberNamesErrorReturn::MemberName(error))`, the refusal unchanged |
| admitted | every name is admitted | none | the constructor, once per name | `Ok` of the admitted names in order |

Every name is a distinct lowercase identifier the constructor admits, other than `line`, so no input takes the refusal.

## `possession_vectors`

`possession_vectors<'a, P: IPairingArithmetic, K: IKeyAgreementAdapter<Pairing = P>>(deps: &PossessionVectorsDeps<'a, P, K>, params: PossessionVectorsParams, payload: PossessionVectorsPayload) -> PossessionVectorsReturn`, the trusted form: every input is a typed value from the generate concrete. The function reads no clock and touches no filesystem; it names no curve, concrete, tag, or count; the count arrives in its params, the proofs' challenges are the key agreement's own under the tags its declaration carries, and every encoding is the selected pairing's.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| fill refused | `deps.random.fill_bytes` returns `Err(error)` on a draw | the call's return | `deps.random.fill_bytes(FillBytesParams, FillBytesPayload { length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH })` | `Err(PossessionVectorsErrorReturn::FillBytes(error))`, the error unchanged |
| generation refused | `deps.key_agreement.generate_keys` returns `Err(error)` on a record | the call's return | `deps.key_agreement.generate_keys(GenerateKeysParams, GenerateKeysPayload { x_uniform, y_uniform })` | `Err(PossessionVectorsErrorReturn::GenerateKeys(error))`, the error unchanged |
| scalar length | `<[u8; 32]>::try_from(success.bytes.expose().as_ref())` fails on an encoded scalar | the conversion's return | `deps.pairing.encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar })` | `Err(PossessionVectorsErrorReturn::ScalarLength { actual })`, `actual` the encoding's length |
| computed | every draw and every generation succeeds | none | the calls each subsection states | `Ok(PossessionVectorsSuccessReturn { records })`, the records in draw order |

### Scalar bytes

For any scalar, dependency call `deps.pairing.encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar })`, unpacked irrefutably; condition `<[u8; 32]>::try_from(success.bytes.expose().as_ref())` fails, outcome `Err(PossessionVectorsErrorReturn::ScalarLength { actual })`, `actual` the encoding's length; otherwise the array. Every concrete encodes a scalar to 32 bytes, the family's mock concrete included, so no input takes the refusal.

### Draws

For each index from `0` below `params.count.get()`: two dependency calls `deps.random.fill_bytes(FillBytesParams, FillBytesPayload { length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH })`, held as `x_uniform` then `y_uniform`, a failure returning `Err(PossessionVectorsErrorReturn::FillBytes(error))` unchanged.

### Generation

Dependency call `deps.key_agreement.generate_keys(GenerateKeysParams, GenerateKeysPayload { x_uniform, y_uniform })`; a failure returns `Err(PossessionVectorsErrorReturn::GenerateKeys(error))` unchanged.

### Components

Dependency calls `deps.key_agreement.key_pair_components(KeyPairComponentsParams, KeyPairComponentsPayload { key_pair: &success.key_pair })`, `key_pair_public_keys(KeyPairPublicKeysParams, KeyPairPublicKeysPayload { key_pair: &success.key_pair })`, and `possession_components(PossessionComponentsParams, PossessionComponentsPayload { possession: &success.possession })`, each unpacked irrefutably; outcome a `PossessionVector` with `x` and `y` the scalar bytes of clones of `components.x.expose()` and `components.y.expose()`, `pk1` and `r1` the `encode_g1` bytes of `pk1` and `r1`, `pk2` and `r2` the `encode_g2` bytes of `pk2` and `r2`, and `z1` and `z2` the scalar bytes of `z1` and `z2`.

### Ordering

Per record, both draws, the generation, then the components; the first failure returns; every drawn uniform and every secret component's `Secret` is cleared when it drops; `payload` is not read; no side effect beyond the draws.

## Integration: callee dispositions

- `a_source_from_the_factory_draws_random_bytes_through_the_family_trait` (`random/factory`): absorbed, proven by `every_run_emits_the_configured_count_of_records_with_distinct_secrets` in the private integration test element `integration_test.rs`; restated in this node's terms as: each record's `x_uniform` and `y_uniform` are two draws of the source `create_random_source` builds, made through `IRandomSourceAdapter::fill_bytes` in draw order.
- `a_sampled_scalar_encodes_through_secret` (`pairing/factory`): absorbed, proven by `every_record_encodes_its_scalars_in_32_bytes_and_its_points_in_the_pairings_widths`; restated in this node's terms as: each record's `x`, `y`, `z1`, and `z2` are `encode_scalar`'s bytes of the secrets and responses, read through `Secret::expose`, each a 32-byte array.
- `every_key_agreement_forms_the_public_keys_its_secrets_form` (`envelope/factory`): absorbed, proven by `every_record_carries_public_keys_that_are_the_generators_times_their_secrets`; restated in this node's terms as: each record's `pk1` and `pk2` are the public keys of the key pair whose secrets are its `x` and `y`.
- `public_keys_are_admitted_with_their_own_proof_of_possession` and `a_possession_rebuilt_from_its_components_is_admitted` (`envelope/factory`): absorbed, proven by `every_record_carries_proofs_the_key_agreement_admits`; restated in this node's terms as: the keys and proof components each record carries are admitted together by `public_keys_from_components`, and a possession rebuilt from a record's `r1`, `z1`, `r2`, and `z2` is the possession the adapter issued.

## Integration: own entries

Each entry is proven by the block of the private integration test element `integration_test.rs` of the same name.

- `every_run_emits_the_configured_count_of_records_with_distinct_secrets`: condition `possession_vectors` over a count greater than one, through the adapters the runner hands the case consumer, the source `create_random_source` builds for `RandomSourceKind::OperatingSystem`; outcome `Ok` whose `records` hold the configured count and no two records share an `x` or a `y`; variation every member of `PAIRING_CONCRETES`, each over every member of `DECLARED_KEY_AGREEMENT_SELECTIONS` with every selection of `DECLARED_ENCODING_SELECTIONS` and `DECLARED_HASH_TO_SCALAR_SELECTIONS`; edge that must survive the composition: each record's two draws are the operating system generator's, so the secrets differ between records.
- `every_record_encodes_its_scalars_in_32_bytes_and_its_points_in_the_pairings_widths`: condition `possession_vectors` over a count greater than one, run as in `every_run_emits_the_configured_count_of_records_with_distinct_secrets`; outcome `Ok`, each record's `pk1` and `r1` holding the length of `encode_g1` of `g1_generator` and its `pk2` and `r2` the length of `encode_g2` of `g2_generator` through the pairing the consumer holds, and its `x`, `y`, `z1`, and `z2` each admitted by the pairing's `decode_scalar`; variation as `every_run_emits_the_configured_count_of_records_with_distinct_secrets`, the widths differing between the BN254 and BLS12-381 concretes, so an encoding fixed to one curve's width fails; edge that must survive the composition: every concrete in `PAIRING_CONCRETES` encodes a scalar to 32 bytes, so no run takes the scalar-length refusal.
- `every_record_carries_public_keys_that_are_the_generators_times_their_secrets`: condition `possession_vectors` as in `every_record_encodes_its_scalars_in_32_bytes_and_its_points_in_the_pairings_widths`; outcome each record's `pk1` equals `encode_g1` of `mul_g1` of `g1_generator` by the scalar `decode_scalar` returns for its `x`, and its `pk2` equals `encode_g2` of `mul_g2` of `g2_generator` by the scalar `decode_scalar` returns for its `y`; variation as `every_run_emits_the_configured_count_of_records_with_distinct_secrets`, the secrets differing from each other, so swapped secrets or swapped keys fail; edge that must survive the composition: the keys and secrets are read from one key pair the adapter formed over the pairing the consumer holds.
- `every_record_carries_proofs_the_key_agreement_admits`: condition `possession_vectors` over a count greater than one, then `public_keys_from_components` over `PublicKeysComponents { pk1, pk2 }` decoded from a record's keys through `decode_g1` and `decode_g2` and the possession `possession_from_components` returns for `PossessionComponents { r1, z1, r2, z2 }` decoded from the same record; outcome `Ok` for every record, and `Err(PublicKeysFromComponentsErrorReturn::PossessionG1)` for one record's keys paired with another record's possession; variation as `every_run_emits_the_configured_count_of_records_with_distinct_secrets`, the proofs of two records differing, so an admission that ignores the proof fails; edge that must survive the composition: the proof verifies under the challenge the adapter computes itself, from the components the record carries.
- `every_record_survives_the_canonical_encoding_under_the_description`: condition a record that differs in every field from every other field, encoded under `PossessionVectorDescription` through the encoder `create_encoding` hands the consumer, and the bytes decoded under the same description; outcome the decoded value equals the record; variation every member of `DECLARED_ENCODING_SELECTIONS`, whose byte forms differ, so a concrete whose decode returns any value its bytes do not hold fails; edge that must survive the composition: every kind in `POSSESSION_VECTOR_FIELD_KINDS` is a kind each concrete encodes, and `to_fields` emits values of the kinds the description declares, so the encoder refuses nothing the description emits.

## Integration: private surface

The chain of real functions `run_possession_vectors_case`, `create_pairing`, `create_hash_to_scalar`, the selected concrete's `hash_to_scalar`, `create_encoding`, the selected encoding concrete's `encode` and `decode`, `create_random_source` for `RandomSourceKind::OperatingSystem`, `create_key_agreement`, the selection's concrete through its `try_new`, `possession_vectors`, the adapter methods the entries name, and `PossessionVectorDescription`'s `try_new`, `to_fields`, and `fields_to_value`, with `Secret` of `domain` running for real; the cases are every member of `PAIRING_CONCRETES`, each over every member of `DECLARED_KEY_AGREEMENT_SELECTIONS` with every selection of `DECLARED_ENCODING_SELECTIONS` and `DECLARED_HASH_TO_SCALAR_SELECTIONS` handed to the real concretes, and, for the round trip, every member of `DECLARED_ENCODING_SELECTIONS`; no outer-edge collaborator is mocked, the operating system's generator behind `create_random_source` running real; the observable result is `possession_vectors`'s `Result`, its records, and the scalars and points the entries compare through `decode_scalar`, `decode_g1`, `decode_g2`, `encode_g1`, and `encode_g2`; the entries proven are the own entries above and the absorbed entries.
