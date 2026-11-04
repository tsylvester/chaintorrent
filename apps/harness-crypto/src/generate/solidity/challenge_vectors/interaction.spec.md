# `challenge_vectors` — interaction spec

Branch contract for the `challenge_vectors` module of the `solidity` module of the `generate` module of `apps/harness-crypto`: `challenge_vectors` emits the challenge vector records, a drawn kind — for each of the configured count, a standalone mint transcript and a standalone transfer transcript over clones of the typed mint and transfer contexts the payload carries, each transcript's purpose read from its context and its every point a generator times a scalar the function draws, each record carrying its purpose, its draws, the transcript's encoding through its description and the encoding family, and its challenge, the hash-to-scalar mapping of that encoding under the challenge tag the delivery-proof declaration carries — beside the record description and its member names and the canonical delivery context with its field vocabulary. Each function's branches state condition, decision, dependency call, and the exact return outcome.

## `ChallengeVectorDescription::try_new`

`try_new(_params: ChallengeVectorDescriptionConstructorParams) -> ChallengeVectorDescriptionTryNewReturn`: one branch; outcome `Ok(ChallengeVectorDescription)`.

## `IEncodingContract` for `ChallengeVectorDescription`

`type Described = ChallengeVector`, `type FromFieldsErrorReturn = ChallengeVectorFromFieldsErrorReturn`, `FIELDS` `&CHALLENGE_VECTOR_FIELD_KINDS`.

`to_fields(params: ToFieldsParams, payload: &ChallengeVector) -> ToFieldsReturn`: one branch; outcome `Ok(ToFieldsSuccessReturn { fields: CanonicalFields { values } })`, `values` being `CanonicalFieldValue::Unsigned16` of `purpose`, `CanonicalFieldValue::Bytes` of a clone of `draws`, `CanonicalFieldValue::Bytes` of a clone of `encoding`, and `CanonicalFieldValue::Unsigned256` of `challenge`, in member order.

`fields_to_value(params: FromFieldsParams, payload: CanonicalFields) -> FromFieldsReturn<ChallengeVector, ChallengeVectorFromFieldsErrorReturn>`:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| fields refused | `CanonicalFieldReader::try_new`, the `read_unsigned16`, a `read_bytes`, or the `read_unsigned256` returns `Err(error)` | the call's return | `CanonicalFieldReader::try_new(payload, Self::FIELDS)`, then `read_unsigned16` for `purpose`, `read_bytes` for `draws` and `encoding`, and `read_unsigned256` for `challenge`, in that order | `Err(ChallengeVectorFromFieldsErrorReturn::Fields(error))`, the refusal unchanged |
| rebuilt | the reader's construction and every read return `Ok` | none | the same calls | `Ok(FromFieldsSuccessReturn { described: ChallengeVector { purpose, draws, encoding, challenge } })`, each the value its read returned |

## `member_names`

`member_names(&self) -> ChallengeVectorMemberNamesReturn`: for each index in order, dependency call `SolidityMemberName::try_new(SolidityMemberNameConstructorParams { text })` over `CHALLENGE_VECTOR_MEMBER_NAMES` at that index.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| refused | `SolidityMemberName::try_new` returns `Err(error)` at an index | the constructor's return | the constructor, once per name in order | `Err(ChallengeVectorMemberNamesErrorReturn::MemberName(error))`, the refusal unchanged |
| admitted | every name is admitted | none | the constructor, once per name | `Ok` of the admitted names in order |

Every name is a lowercase identifier the constructor admits, other than `line`, so no input takes the refusal.

## `challenge_vectors`

`challenge_vectors<'a, P: IPairingAdapter, E: IEncoderAdapter, F: IChainForms>(deps: &ChallengeVectorsDeps<'a, P, E>, params: ChallengeVectorsParams, payload: ChallengeVectorsPayload<'_, F>) -> ChallengeVectorsReturn`, the trusted form: `F` is the function's own type parameter, since the contexts it is handed and the transcripts it builds and encodes are generic in it, and its return names no `F`; every input is a typed value from the generate concrete. The function reads no clock and touches no filesystem; it names no curve, concrete, form, tag, count, chain, purpose, or context value; the count and version arrive in its params, the tag in the declaration, the typed contexts, each carrying its purpose, in its payload, and every encoding is the selected pairing's and the encoding family's.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| unsupported version | `params.statement_version` is not `DELIVERY_STATEMENT_VERSION_ONE` | equality, before any other step | none | `Err(ChallengeVectorsErrorReturn::UnsupportedStatementVersion { version: params.statement_version })` |
| tag refused | `DomainTag::try_new` returns `Err(error)` on the declared tag | the constructor's return | `DomainTag::try_new(DomainTagConstructorParams { bytes })` over a copy of `payload.delivery_proof.challenge_tag` | `Err(ChallengeVectorsErrorReturn::DomainTag(error))`, the refusal unchanged |
| fill refused | `deps.random.fill_bytes` returns `Err(error)` on a draw | the call's return | `deps.random.fill_bytes(FillBytesParams, FillBytesPayload { length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH })` | `Err(ChallengeVectorsErrorReturn::FillBytes(error))`, the error unchanged |
| sampling refused | `<P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes` returns `Err(error)` on a drawn uniform | the call's return | `sample_from_uniform_bytes(SampleUniformScalarParams, SampleUniformScalarPayload { uniform: success.bytes })` | `Err(ChallengeVectorsErrorReturn::SampleScalar(error))`, the error unchanged |
| encoding refused | `deps.encoder.encode` returns `Err(error)` on a record | the call's return | `deps.encoder.encode(EncodeParams { description: &description }, &statement)` | `Err(ChallengeVectorsErrorReturn::Encoding(error))`, the error unchanged |
| hash refused | `deps.hash_to_scalar.hash_to_scalar` returns `Err(error)` on a record | the call's return | `deps.hash_to_scalar.hash_to_scalar(HashToScalarParams { tag: &tag }, HashToScalarPayload { message: &bytes })` | `Err(ChallengeVectorsErrorReturn::HashToScalar(error))`, the error unchanged |
| scalar length | `<[u8; 32]>::try_from(success.bytes.expose().as_ref())` fails on an encoded scalar | the conversion's return | `deps.pairing.encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar })` | `Err(ChallengeVectorsErrorReturn::ScalarLength { actual })`, `actual` the encoding's length |
| computed | the version serves, the tag admits, and every draw, sampling, encoding, and hash succeeds | none | the calls each subsection states | `Ok(ChallengeVectorsSuccessReturn { records })`, the records in index order, each index's mint record before its transfer record |

### The tag

Dependency call `DomainTag::try_new(DomainTagConstructorParams { bytes })` over a copy of `payload.delivery_proof.challenge_tag`; a refusal returns `Err(ChallengeVectorsErrorReturn::DomainTag(error))` unchanged, before any draw.

### Generators

Dependency calls `deps.pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload)` and `deps.pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload)`, each unpacked irrefutably, once.

### Scalar bytes

For any scalar, dependency call `deps.pairing.encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar })`, unpacked irrefutably; condition `<[u8; 32]>::try_from(success.bytes.expose().as_ref())` fails, outcome `Err(ChallengeVectorsErrorReturn::ScalarLength { actual })`, `actual` the encoding's length; otherwise the array. Every concrete encodes a scalar to 32 bytes, the family's mock concrete included, so no input takes the refusal and it has no unit test.

### Point draw, for a group

Dependency call `deps.random.fill_bytes(FillBytesParams, FillBytesPayload { length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH })`, a failure returning `Err(ChallengeVectorsErrorReturn::FillBytes(error))` unchanged; then `<P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(SampleUniformScalarParams, SampleUniformScalarPayload { uniform: success.bytes })`, a failure returning `Err(ChallengeVectorsErrorReturn::SampleScalar(error))` unchanged; then `deps.pairing.mul_g1(MulG1Params, MulG1Payload { point, scalar })` over a clone of the first-group generator for a first-group point, or `mul_g2` over a clone of the second-group generator for a second-group point, the scalar a clone of `success.scalar.expose()`, unpacked irrefutably; the scalar bytes of another clone appended to the record's draws; outcome the product.

### Mint record

For each index from `0` below `params.count.get()`: eleven point draws in the description's group-field order — the buyer keys' `pk1` in the first group and `pk2` in the second, the envelope's `c1` and `c2` in the first and `d1` and `d2` in the second, and the first messages' `hpub` in the second, `c1` and `c2` in the first, and `d1` and `d2` in the second; the statement `MintStatement { suite_identifier, chain, entitlement_contract, asset_root, parameter_set_identifier, source_entitlement, target_entitlement, old_interval, new_interval, purpose, seller, buyer, buyer_keys: PublicKeysComponents { pk1, pk2 }, envelope: EnvelopeComponents { c1, c2, d1, d2 }, expiry, first_messages: MintFirstMessages { hpub, c1, c2, d1, d2 } }`, every context field, `purpose` among them, moved from the record's own `payload.mint.clone()`.

### Transfer record

For the same index after its mint record: eighteen point draws in the description's group-field order — the seller keys' `pk1` in the first group and `pk2` in the second, the buyer keys' likewise, the old envelope's `c1` and `c2` in the first and `d1` and `d2` in the second, the new envelope's likewise, and the first messages' `pk1`, `c1`, and `c2` in the first and `pk2`, `d1`, and `d2` in the second; the statement `TransferStatement { suite_identifier, chain, entitlement_contract, asset_root, parameter_set_identifier, source_entitlement, target_entitlement, old_interval, new_interval, purpose, seller, buyer, seller_keys, buyer_keys, old_envelope, new_envelope, expiry, first_messages: TransferFirstMessages { pk1, c1, c2, pk2, d1, d2 } }`, every context field, `purpose` among them, moved from the record's own `payload.transfer.clone()`.

### Encoding, per record

The description `MintStatementDescription::<P, F>::try_new(MintStatementDescriptionConstructorParams { pairing: deps.pairing })` for a mint record or `TransferStatementDescription::<P, F>::try_new(TransferStatementDescriptionConstructorParams { pairing: deps.pairing })` for a transfer record, unpacked irrefutably; dependency call `deps.encoder.encode(EncodeParams { description: &description }, &statement)`; a failure returns `Err(ChallengeVectorsErrorReturn::Encoding(error))` unchanged.

### Challenge, per record

Dependency call `deps.hash_to_scalar.hash_to_scalar(HashToScalarParams { tag: &tag }, HashToScalarPayload { message: &bytes })` over the encoding; a failure returns `Err(ChallengeVectorsErrorReturn::HashToScalar(error))` unchanged; outcome a `ChallengeVector` with `purpose` the `code()` of the purpose the record's statement holds, `draws` the concatenated scalar bytes, `encoding` the encoded bytes, and `challenge` the scalar bytes of the mapped scalar.

### Ordering

The version, the tag, the generators, then the records; within a record its draws in group-field order, then its encoding, then its challenge; the first failure returns; every drawn uniform's and scalar's `Secret` is cleared when it drops; no side effect beyond the draws.

## Integration: callee dispositions

- `chain_forms_identity_survives_its_field_round_trip`, `chain_forms_entitlement_survives_its_field_round_trip`, `chain_forms_interval_survives_its_field_round_trip`, and `chain_forms_chain_identifier_survives_its_field_round_trip` (`chain/create_chain_forms`): absorbed, proven by `challenge_vectors_place_each_contexts_configured_values_in_its_transcript` in the private integration test element `integration_test.rs`, which produces each form through its `from_field` in its arrangement and reads it back through its `to_field` in the encoding `challenge_vectors` returns, over `CHAIN_FORMS_CONCRETES`.
- `a_sampled_scalar_encodes_through_secret` (`pairing/factory`): absorbed, proven by `challenge_vectors_draw_every_transcript_point_from_its_recorded_scalar` in the private integration test element `integration_test.rs`; restated in this node's terms as: the scalar `sample_from_uniform_bytes` returns for a drawn uniform encodes through `encode_scalar` to the 32 bytes the record holds, and the point built from it is the transcript's.
- `a_sampling_refusal_reaches_the_caller_unchanged` (`pairing/factory`): carried; restated in this node's terms as: a sampling refusal reaches the caller of `challenge_vectors` whole as `ChallengeVectorsErrorReturn::SampleScalar`; route `sample_from_uniform_bytes`, then `challenge_vectors`; the outer-edge collaborator on the route is the pairing family's mock concrete selected by `PairingConcrete::Mock(MockIPairingAdapterFailureMode::SampleScalarWrongLength)`.
- `every_concrete_declaring_the_keccak256_identifier_hashes_the_reference_inputs_to_the_oracle` and `every_concrete_declaring_the_keccak256_identifier_hashes_the_context_the_encoding_factory_encodes_to_the_oracle` (`hash-to-scalar/factory`): absorbed, proven by `challenge_vectors_record_the_challenge_the_declared_tag_maps_each_encoding_to` in the private integration test element `integration_test.rs`; restated in this node's terms as: the adapter hashes the bytes the encoder returned for a statement, under the declared tag, to the scalar the oracle states.
- `every_concrete_returns_the_sampling_bound_refusal_whole` (`hash-to-scalar/factory`): absorbed, proven by `challenge_vectors_return_the_hash_to_scalar_refusal_whole` in the private integration test element `integration_test.rs`.
- `an admitted tag reads back through the crate root` (`domain_tag`, recorded by `hash-to-scalar/factory`): absorbed, proven by `challenge_vectors_record_the_challenge_the_declared_tag_maps_each_encoding_to` in the private integration test element `integration_test.rs`, whose oracle hashes under the tag `DomainTag::try_new` admits for the declared bytes.
- `a refusal reaches the caller whole and yields no tag` (`domain_tag`, recorded by `hash-to-scalar/factory`): carried; restated in this node's terms as: a declared tag the constructor refuses yields no record, and the caller receives `ChallengeVectorsErrorReturn::DomainTag` whole; route `DomainTag::try_new`, then `challenge_vectors`; the route reaches no outer-edge collaborator.
- `every concrete decodes what it encodes` (`encoding/factory`): absorbed, proven by `challenge_vectors_place_each_contexts_configured_values_in_its_transcript` in the private integration test element `integration_test.rs`; restated in this node's terms as: the bytes `encode` returns for a statement under its description decode through `decode` under the same description to the transcript the statement holds, over `DECLARED_ENCODING_SELECTIONS`.

## Integration: own entries

Each entry is proven by the block of the private integration test element `integration_test.rs` of the same name.

- `challenge_vectors_emit_a_mint_and_a_transfer_record_per_count_in_order`: condition `challenge_vectors` is called with a `count` above one, `DELIVERY_STATEMENT_VERSION_ONE`, and a mint and a transfer context at their default values carrying the purposes `DELIVERY_PURPOSE_REPLACEMENT` and `DELIVERY_PURPOSE_GRANT`, over the real pairing, encoder, forms, and hash-to-scalar adapter and the operating-system generator; outcome `Ok` whose `records` hold, for each index in order, a mint record then a transfer record, each `purpose` the code of its purpose, each `draws` one 32-byte scalar encoding per point of its transcript, eleven for a mint record and eighteen for a transfer record, and no two records sharing `draws`; variation a count above one, purposes other than the builder defaults, over every member of `PAIRING_CONCRETES`, so a function that fixes a purpose, orders by relation, emits per relation instead of per index, or repeats draws fails; edge that must survive the composition: the draws are the real generator's, drawn per record.
- `challenge_vectors_place_each_contexts_configured_values_in_its_transcript`: condition `challenge_vectors` is called with a mint context and a transfer context whose every value differs from every other, each form produced by its form's `from_field` over a `CanonicalFieldValue` of the kind the real form declares, and purposes other than `DELIVERY_PURPOSE_MINT` and `DELIVERY_PURPOSE_TRANSFER`; outcome each record's `encoding`, decoded through the encoder family under the record's statement description, holds the context's configured values at the canonical field indices of the chain identifier, entitlement contract, source and target entitlements, old and new intervals, purpose, seller, buyer, and expiry; variation distinct values per field and per context, over every member of `PAIRING_CONCRETES`, `DECLARED_ENCODING_SELECTIONS`, and `CHAIN_FORMS_CONCRETES`, so a function that swaps two fields, reads the other relation's context, or substitutes a default fails; edge that must survive the composition: a form the caller admitted reaches the encoding unaltered through the form's `to_field` and the description.
- `challenge_vectors_draw_every_transcript_point_from_its_recorded_scalar`: condition `challenge_vectors` is called as in the first entry; outcome for each record, the scalar each 32-byte chunk of `draws` decodes to through the pairing's `decode_scalar`, multiplied onto the generator of the group of the transcript field at the same position in group-field order and encoded through the pairing, equals the decoded transcript's value at that field; variation every record, over every member of `PAIRING_CONCRETES`, so a point drawn in the wrong group, drawn from a scalar the record does not hold, or recorded out of order fails; edge that must survive the composition: the record's draws reproduce every point of its transcript.
- `challenge_vectors_record_the_challenge_the_declared_tag_maps_each_encoding_to`: condition `challenge_vectors` is called as in the first entry with a declaration whose `challenge_tag` differs from every other tag; outcome each record's `challenge` equals the 32-byte encoding of the scalar the pairing's `sample_from_uniform_bytes` returns for `expected_uniform_bytes` over the declared tag and the record's `encoding`; variation every record, over every member of `PAIRING_CONCRETES` and every case of `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declares `HashToScalarIdentifier::Keccak256V1`, so a challenge computed under another tag, over other bytes, or not mapped fails; edge that must survive the composition: the bytes the encoder returns are the bytes hashed, on both curves.
- `challenge_vectors_return_the_hash_to_scalar_refusal_whole`: condition `challenge_vectors` is called as in the first entry with the hash-to-scalar adapter the factory returns for `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::SamplingRefused)`; outcome `Err(ChallengeVectorsErrorReturn::HashToScalar(HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::WrongLength { expected: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH, actual: 0 })))`; variation the mock selection under that failure mode over every member of `PAIRING_CONCRETES`, the case that reaches this arm; edge that must survive the composition: the refusal arrives whole after the first record's draws and encoding.

## Integration: private surface

The chain of real functions `run_challenge_vectors_case` driving `create_pairing`, the arm's concrete through its `try_new`, `create_hash_to_scalar`, `create_random_source` for `RandomSourceKind::OperatingSystem`, `create_encoding`, and `create_chain_forms`, each level's consume method, `consume_challenge_vectors_case`, `challenge_vectors`, the statement descriptions' `try_new`, `to_fields`, and `fields_to_value`, the encoder's `encode` and `decode`, the forms' `from_field` and `to_field`, and the pairing's `decode_scalar`, `mul_g1`, `mul_g2`, `encode_g1`, `encode_g2`, and `encode_scalar`, with `Secret` of `domain` running for real; the outer-edge collaborator mocked is the hash-to-scalar family's mock concrete under `SamplingRefused`, for the hash-to-scalar refusal entry alone, the operating system's generator running real in every other entry; the observable result is the `Result` `challenge_vectors` returns, the records' `draws`, `encoding`, and `challenge`, and the values the encoder decodes the encodings to; the entries proven are every own entry above and the absorbed entries.
