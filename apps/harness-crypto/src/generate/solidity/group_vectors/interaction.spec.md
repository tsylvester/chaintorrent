# `group_vectors` — interaction spec

Branch contract for the `group_vectors` module of the `solidity` module of the `generate` module of `apps/harness-crypto`: `group_vectors` emits the pairing library's group-operation and pairing-check vector records, generic over the selected pairing — first-group addition, first-group multiplication, and the pairing check on every curve, and second-group addition and both groups' multi-scalar multiplication where the pairing declares second-group arithmetic at the verifier; each record carries its drawn scalars, the precompile's input, and the reference's output. Each function's branches state condition, decision, dependency call, and the exact return outcome.

## `VectorCount::try_new`

`try_new(params: VectorCountConstructorParams) -> VectorCountTryNewReturn`:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| zero | `params.count` is `0` | `NonZeroU32::new(params.count)` is `None` | none | `Err(VectorCountTryNewErrorReturn::Zero)` |
| admitted | `params.count` is positive | `NonZeroU32::new(params.count)` is `Some` | none | `Ok(VectorCount { count })` |

`VectorCount::get(&self) -> u32` returns `self.count.get()`.

## `MsmTermCount::try_new`

`try_new(params: MsmTermCountConstructorParams) -> MsmTermCountTryNewReturn`: the same branches with `MsmTermCountTryNewErrorReturn::Zero`; `MsmTermCount::get(&self) -> u32` returns `self.count.get()`.

## `GroupOperationVectorDescription::try_new` and `PairingCheckVectorDescription::try_new`

`try_new(_params) -> GroupOperationVectorDescriptionTryNewReturn` and `try_new(_params) -> PairingCheckVectorDescriptionTryNewReturn`: one branch each; outcome `Ok` of the fieldless struct.

## `IEncodingContract for GroupOperationVectorDescription`

`type Described = GroupOperationVector`, `type FromFieldsErrorReturn = GroupOperationVectorFromFieldsErrorReturn`, `FIELDS` `&GROUP_OPERATION_VECTOR_FIELD_KINDS`.

`to_fields`: one branch; outcome `Ok(ToFieldsSuccessReturn { fields: CanonicalFields { values } })`, `values` being `CanonicalFieldValue::Bytes` of clones of `draws`, `input`, and `output` in that order.

`fields_to_value`:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| count | `payload.values` does not convert into `[CanonicalFieldValue; VECTOR_RECORD_FIELD_COUNT]` | the conversion | none | `Err(GroupOperationVectorFromFieldsErrorReturn::FieldCount { expected: VECTOR_RECORD_FIELD_COUNT, actual })`, `actual` the vector's length |
| kind | the lowest index `i` whose `kind()` is not `CanonicalFieldKind::Bytes` | that field's kind | none | `Err(GroupOperationVectorFromFieldsErrorReturn::FieldKind { index: i, expected: CanonicalFieldKind::Bytes })` |
| rebuilt | the count converts and every field is `Bytes` | none | none | `Ok(FromFieldsSuccessReturn { described: GroupOperationVector { draws, input, output } })`, each moved from its field |

## `IEncodingContract for PairingCheckVectorDescription`

`type Described = PairingCheckVector`, `type FromFieldsErrorReturn = PairingCheckVectorFromFieldsErrorReturn`, `FIELDS` `&PAIRING_CHECK_VECTOR_FIELD_KINDS`.

`to_fields`: one branch; `values` being `CanonicalFieldValue::Bytes` of clones of `draws` and `input` and `CanonicalFieldValue::FixedBytes32` of `output`.

`fields_to_value`: the same count branch with `PairingCheckVectorFromFieldsErrorReturn`, then the kind branch by the lowest index whose value is not the kind `PAIRING_CHECK_VECTOR_FIELD_KINDS` holds at that index, carrying that kind as `expected`, then the rebuilt branch.

## `member_names`

`member_names(&self) -> VectorRecordMemberNamesReturn` on each description: for each index in order, dependency call `SolidityMemberName::try_new(SolidityMemberNameConstructorParams { text })` over `VECTOR_RECORD_MEMBER_NAMES` at that index; a refusal returns `Err(VectorRecordMemberNamesErrorReturn::MemberName(error))` unchanged; otherwise `Ok` of the admitted names in order. Every name is a lowercase identifier the constructor admits, other than `line`, so no input takes the refusal.

## `group_vectors`

`group_vectors<'a, P: IPairingArithmetic>(deps: &GroupVectorsDeps<'a, P>, params: GroupVectorsParams, payload: GroupVectorsPayload) -> GroupVectorsReturn`, the trusted form: every input is a typed value from the generate concrete. The function reads no clock and touches no filesystem; it names no curve, concrete, kind count, file, library, struct, or function name; every count and term count arrives in its params, and every encoding is the selected pairing's.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| second-group settings for a first-group-only pairing | `P::DECLARATION.verifier_group_arithmetic` is `FirstGroupOnly` and `params.second_group` is `Some(_)` | an exhaustive `match` over the declaration and `params.second_group`, before any draw | none | `Err(GroupVectorsErrorReturn::SecondGroupSettingsForFirstGroupOnly)` |
| second-group settings missing | the declaration is `BothGroups` and `params.second_group` is `None` | the same `match` | none | `Err(GroupVectorsErrorReturn::SecondGroupSettingsMissing)` |
| fill refused | a draw's `fill_bytes` returns `Err(error)` | the call's return | `deps.random.fill_bytes(FillBytesParams, FillBytesPayload { length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH })` | `Err(GroupVectorsErrorReturn::FillBytes(error))`, the refusal unchanged |
| sampling refused | a draw's `sample_from_uniform_bytes` returns `Err(error)` | the call's return | `<P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(SampleUniformScalarParams, SampleUniformScalarPayload { uniform })` | `Err(GroupVectorsErrorReturn::SampleScalar(error))`, the refusal unchanged |
| emitted | `FirstGroupOnly` with `None`, or `BothGroups` with `Some(settings)`, and every draw and sampling succeeds | none | the calls each subsection states | `Ok(GroupVectorsSuccessReturn { first_group, second_group })` |

### A draw

Dependency call `deps.random.fill_bytes(FillBytesParams, FillBytesPayload { length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH })`; a failure returns the fill-refused branch. Then `<P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(SampleUniformScalarParams, SampleUniformScalarPayload { uniform: success.bytes })`; a failure returns the sampling-refused branch. The drawn scalar is `success.scalar.expose().clone()`, and its record bytes are `deps.pairing.encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar })`'s `bytes.expose().as_ref()`, appended to the record's `draws`. Every draw takes this branch structure, through one local closure over `deps` that returns the scalar or the function's error.

### The generators

`g1` from `deps.pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload)` and `g2` from `g2_generator`, each unpacked irrefutably. A multiple `a·g` is `mul_g1` or `mul_g2` over a clone of the generator and a clone of `a`; an encoding is `encode_g1`, `encode_g2`, or `encode_scalar`'s bytes. Every adapter call other than the draw returns `Result<_, Infallible>` and is unpacked irrefutably.

### First-group addition

For each index from `0` below `params.first_group.g1_add.get()`: draws `a` then `b`; `left` is `a·g1` and `right` is `b·g1`; `sum` from `add_g1(AddG1Params, AddG1Payload { left, right })`, over clones; outcome a `GroupOperationVector` with `input` the encodings of `left` then `right` and `output` the encoding of `sum`.

### First-group multiplication

For each index below `g1_mul.get()`: draws `a` then `s`; `point` is `a·g1`; `product` from `mul_g1(MulG1Params, MulG1Payload { point, scalar: s })`; `input` the encoding of `point` then the encoding of `s`; `output` the encoding of `product`.

### The pairing check

For each index below `pairing_check.get()`: draws `a` then `b`; condition the index is even, decision `index % 2 == 0`, `t` is `neg_scalar` of `mul_scalar(a, b)`; otherwise draws `c` and `t` is `c`; terms `PairingProductTerm { g1: a·g1, g2: b·g2 }` and `PairingProductTerm { g1: t·g1, g2: g2 }`; `is_one` from `pairing_product_is_one(PairingProductIsOneParams, PairingProductIsOnePayload { terms })` over clones; `input` the encodings of `a·g1`, `b·g2`, `t·g1`, and `g2` in that order; `output` the 32-byte word of zeros whose last byte is `0x01` where `is_one` and `0x00` otherwise; outcome a `PairingCheckVector`.

### The second group

Only with `Some(settings)`: second-group addition for each index below `settings.g2_add.get()`, the first-group addition branch over `g2`, `add_g2`, and `encode_g2`; first-group multi-scalar multiplication for each index below `settings.g1_msm.count.get()`, for each term from `0` below `settings.g1_msm.terms.get()` drawing `a_i` then `s_i`, the term `MsmG1Term { base: a_i·g1, scalar: s_i }`, `input` gaining the base's encoding then `s_i`'s encoding, then `sum` from `msm_g1(MsmG1Params, MsmG1Payload { terms })` and `output` its encoding; second-group multi-scalar multiplication likewise over `settings.g2_msm`, `g2`, `MsmG2Term`, `msm_g2`, and `encode_g2`; outcome `Some(SecondGroupVectors { g2_add, g1_msm, g2_msm })`, and `None` without settings.

### Ordering

The gate, then the kinds in the order `g1_add`, `g1_mul`, `pairing_check`, `g2_add`, `g1_msm`, `g2_msm`, each record's draws in the order above; the first failed draw or sampling returns; every drawn scalar's `Secret` and every clone held in a payload is cleared when it drops; no side effect beyond the draws.

## Integration: callee dispositions

The `pairing/factory` entries this node's route reaches.

- `a_sampling_refusal_reaches_the_caller_unchanged` (`pairing/factory`): carried; in this node's terms a refused sampling is `Err(GroupVectorsErrorReturn::SampleScalar(error))`, the error whole; route `create_pairing`, the arm's concrete's `sample_from_uniform_bytes`, and `group_vectors`; the outer-edge collaborator on the route is the random source whose draw is the sampler's input.
- `a_sampled_scalar_encodes_through_secret` (`pairing/factory`): carried; in this node's terms every sampled scalar's encoding, read through `Secret::expose`, is appended to the record's `draws` in draw order; route `create_pairing`, the arm's concrete's `sample_from_uniform_bytes` and `encode_scalar`, and `group_vectors`; the outer-edge collaborator on the route is the random source whose draw is the sampler's input.

## Integration: own entries

Each entry is proven by the block of the private integration test element `integration_test.rs` of the same name.

- `a_record_survives_its_description_round_trip`: condition each description's `fields_to_value` is called with the fields its `to_fields` returns for a record whose three fields differ from one another in value and length; outcome `Ok` whose `described` equals the record; variation both descriptions, whose output fields differ in kind, so a swapped, dropped, or retyped field fails; edge that must survive the composition: the pairing-check record's output returns as the same 32-byte array.
- `member_names_declare_one_name_per_field`: condition `member_names()` is called on each description; outcome `Ok` of the declared names in order, `draws`, `input`, and `output`, and their count equals the length of the description's `FIELDS`; variation both descriptions, so a names list that does not follow the fields fails; edge that must survive the composition: the names are admitted by the real `SolidityMemberName::try_new`.
- `a_both_groups_pairing_requires_its_second_group_settings`: condition `group_vectors` is called with `second_group` absent for every concrete of `PAIRING_CONCRETES`; outcome `Err(GroupVectorsErrorReturn::SecondGroupSettingsMissing)` where `P::DECLARATION.verifier_group_arithmetic` is `BothGroups`, and `Ok` with `second_group` absent where it is `FirstGroupOnly`; variation every concrete of the declared set, which holds both forms, so a gate that decides by anything other than the declaration fails; edge that must survive the composition: the declaration the gate reads is the real concrete's.
- `a_both_groups_pairing_emits_the_second_group_kinds_at_their_configured_counts`: condition `group_vectors` is called, for every concrete of `PAIRING_CONCRETES`, with `second_group` holding settings where the concrete's declaration is `BothGroups` and absent where it is `FirstGroupOnly`; outcome `second_group` holds exactly the configured records of each kind where the declaration is `BothGroups`, each multi-scalar-multiplication record's `input` holding its configured terms, each a base encoding and a multiplier encoding, and is absent where it is `FirstGroupOnly`; variation the second-group addition count, each multi-scalar-multiplication count, and each term count differ from one another, so swapped kinds or settings fail; edge that must survive the composition: the encodings are the real concrete's.
- `each_first_group_output_is_the_references_result_for_its_drawn_operands`: condition `group_vectors` is called for every concrete of `PAIRING_CONCRETES`, `second_group` absent or present by the declaration, with first-group counts above one; outcome each first-group addition record's `output` is the encoding of `g1` times the sum of its two draws, and each first-group multiplication record's `output` is the encoding of `g1` times the product of its two draws, each reference built through `decode_scalar`, `add_scalar`, `mul_scalar`, `mul_g1`, and `encode_g1`, a path other than the function's; variation every concrete and operating-system draws that differ from record to record, so an output of another operation or another operand fails; edge that must survive the composition: the draws read back are the scalars the records used.
- `each_second_group_output_is_the_references_result_for_its_drawn_operands`: condition `group_vectors` is called on every concrete whose declaration is `BothGroups`; outcome each second-group addition record's `output` is the encoding of `g2` times the sum of its two draws, and each multi-scalar-multiplication record's `output` is the encoding of the group's generator times the sum over its terms of the product of the term's two draws, each reference built through `decode_scalar`, `add_scalar`, `mul_scalar`, the group's multiplication, and its encoder, a path other than the function's; variation term counts above one and operating-system draws, so a sum that drops a term or fixes an operand fails; edge that must survive the composition: the draws read back are the scalars the records used.
- `each_msm_term_carries_its_multiplier_in_its_input`: condition `group_vectors` is called as in `a_both_groups_pairing_emits_the_second_group_kinds_at_their_configured_counts`; outcome for each term of each multi-scalar-multiplication record, the multiplier's encoding in `input` equals the encoding of the term's second draw in `draws`; variation term counts above one, so a multiplier carried from another term or position fails; edge that must survive the composition: the term's base encoding precedes its multiplier encoding.

## Integration: private surface

The chain of real functions `run_group_vectors_case`, `create_pairing` with the arm's real concrete's operations, `create_random_source` with the operating-system source, `group_vectors`, each description's `to_fields` and `fields_to_value`, `member_names()`, and `SolidityMemberName::try_new`; no outer-edge collaborator is mocked, the operating system's generator running real; the observable result is the `GroupVectorsReturn`, the fields, the rebuilt record, and the names list; the entries proven are the own entries above.
