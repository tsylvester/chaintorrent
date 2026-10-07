# group_vectors interactions

`group_vectors` produces the group-operation and pairing-check vector records the Solidity pairing library's Foundry tests prove against, generic over the selected pairing: first-group addition, first-group multiplication, and the pairing check on every pairing, and second-group addition and both groups' multi-scalar multiplication where the pairing declares second-group arithmetic at the verifier. Each record carries the scalars it was drawn from, the precompile's input bytes, and the reference's output. The function reads no clock and touches no filesystem; every count arrives in its params and every encoding is the selected pairing's.

## `VectorCount::try_new`

`VectorCount::try_new(params: VectorCountConstructorParams) -> VectorCountTryNewReturn`.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| zero | `params.count` is zero | `NonZeroU32::new(params.count)` is `None` | none | `Err(VectorCountTryNewErrorReturn::Zero)` |
| admitted | `params.count` is positive | `NonZeroU32::new(params.count)` is `Some(count)` | none | `Ok(VectorCount { count })` |

`VectorCount::get(&self) -> u32` returns `self.count.get()`.

## `MsmTermCount::try_new`

`MsmTermCount::try_new(params: MsmTermCountConstructorParams) -> MsmTermCountTryNewReturn`: the same branches with `MsmTermCountTryNewErrorReturn::Zero`. `MsmTermCount::get(&self) -> u32` returns `self.count.get()`.

## Description constructors

`GroupOperationVectorDescription::try_new(_params: GroupOperationVectorDescriptionConstructorParams) -> GroupOperationVectorDescriptionTryNewReturn` and `PairingCheckVectorDescription::try_new(_params: PairingCheckVectorDescriptionConstructorParams) -> PairingCheckVectorDescriptionTryNewReturn`: one branch each, outcome `Ok` of the fieldless struct.

## `IEncodingContract for GroupOperationVectorDescription`

`type Described = GroupOperationVector`, `type FromFieldsErrorReturn = GroupOperationVectorFromFieldsErrorReturn`, `FIELDS` `&GROUP_OPERATION_VECTOR_FIELD_KINDS`.

`to_fields`:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| mapped | any `GroupOperationVector` | none | none | `Ok(ToFieldsSuccessReturn { fields: CanonicalFields { values } })`, `values` being `CanonicalFieldValue::Bytes` of clones of `draws`, `input`, and `output` in that order |

`fields_to_value`:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| count | `payload.values` does not convert into `[CanonicalFieldValue; VECTOR_RECORD_FIELD_COUNT]` | the conversion | none | `Err(GroupOperationVectorFromFieldsErrorReturn::FieldCount { expected: VECTOR_RECORD_FIELD_COUNT, actual })`, `actual` the vector's length |
| kind | the lowest index `i` whose value is not `CanonicalFieldValue::Bytes` | the per-index kind check | none | `Err(GroupOperationVectorFromFieldsErrorReturn::FieldKind { index: i, expected: CanonicalFieldKind::Bytes })` |
| rebuilt | every field a `Bytes` at its index | none | none | `Ok(FromFieldsSuccessReturn { described: GroupOperationVector { draws, input, output } })`, each moved from its field |

## `IEncodingContract for PairingCheckVectorDescription`

`type Described = PairingCheckVector`, `type FromFieldsErrorReturn = PairingCheckVectorFromFieldsErrorReturn`, `FIELDS` `&PAIRING_CHECK_VECTOR_FIELD_KINDS`.

`to_fields`: one branch, `values` being `CanonicalFieldValue::Bytes` of clones of `draws` and `input` and `CanonicalFieldValue::FixedBytes32` of `output`.

`fields_to_value`: the same count branch with `PairingCheckVectorFromFieldsErrorReturn`, then the kind branch by the lowest index whose value is not the kind `PAIRING_CHECK_VECTOR_FIELD_KINDS` holds at that index, carrying that kind as `expected`, then the rebuilt branch, outcome `Ok(FromFieldsSuccessReturn { described: PairingCheckVector { draws, input, output } })`.

## `members`

`members(&self) -> VectorRecordMembersReturn` on each description: for each index in order, dependency call `SolidityMemberName::try_new(SolidityMemberNameConstructorParams { text })` over `VECTOR_RECORD_MEMBER_NAMES` at that index; a refusal returns `Err(VectorRecordMembersErrorReturn::MemberName(error))` unchanged; each admitted name paired with the description's `FIELDS` kind at the same index as `SolidityRecordMember { name, kind }`; then `SolidityRecordMembers::try_new(SolidityRecordMembersConstructorParams { members })`, a refusal returning `Err(VectorRecordMembersErrorReturn::RecordMembers(error))` unchanged; otherwise `Ok` of the members. Every name is a lowercase identifier the constructor admits, other than `line`, and the names are distinct, so no input takes either refusal, and neither has a unit test.

## `group_vectors`

`group_vectors<'a, P: IPairingArithmetic>(deps: &GroupVectorsDeps<'a, P>, params: GroupVectorsParams, payload: GroupVectorsPayload) -> GroupVectorsReturn`, the trusted form: every input is a typed value from the generate concrete.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| settings on first-group-only | `P::DECLARATION.verifier_group_arithmetic` is `FirstGroupOnly` and `params.second_group` is `Some(_)` | an exhaustive `match` over the declaration and `params.second_group`, before any draw | none | `Err(GroupVectorsErrorReturn::SecondGroupSettingsForFirstGroupOnly)` |
| settings missing | the declaration is `BothGroups` and `params.second_group` is `None` | the same `match` | none | `Err(GroupVectorsErrorReturn::SecondGroupSettingsMissing)` |
| draw failed | `deps.random.fill_bytes(FillBytesParams, FillBytesPayload { length })` returns `Err(error)`, `length` `<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH` | the fill result | `deps.random.fill_bytes`, once per draw | `Err(GroupVectorsErrorReturn::FillBytes(error))`, the error unchanged |
| sampling failed | `<P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(SampleUniformScalarParams, SampleUniformScalarPayload { uniform })` returns `Err(error)`, `uniform` the fill's `success.bytes` | the sampling result | `sample_from_uniform_bytes`, once per draw | `Err(GroupVectorsErrorReturn::SampleScalar(error))`, the error unchanged |
| computed | the gate admits the params and every draw succeeds | the records assembled in the order below | the draw closure and the adapter calls stated | `Ok(GroupVectorsSuccessReturn { first_group: FirstGroupVectors { g1_add, g1_mul, pairing_check }, second_group })` |

`FirstGroupOnly` with `None` and `BothGroups` with `Some(settings)` proceed past the gate.

### Draw

Every draw takes the same branch structure, through one local closure over `deps` that returns the scalar or the function's error: `deps.random.fill_bytes(FillBytesParams, FillBytesPayload { length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH })`, a failure taking the draw-failed branch; then `<P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(SampleUniformScalarParams, SampleUniformScalarPayload { uniform: success.bytes })`, a failure taking the sampling-failed branch; the drawn scalar is `success.scalar.expose().clone()`, and its record bytes are `deps.pairing.encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar })`'s `bytes.expose().as_ref()`, appended to the record's `draws`.

### Generators

`g1` from `deps.pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload)` and `g2` from `deps.pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload)`, each unpacked irrefutably; a multiple `a·g` is `mul_g1` or `mul_g2` over a clone of the generator and a clone of `a`; an encoding is `encode_g1`, `encode_g2`, or `encode_scalar`'s bytes. Every adapter call other than the draw returns `Result<_, Infallible>` and is unpacked irrefutably.

### First-group addition

For each index from `0` below `params.first_group.g1_add.get()`: draws `a` then `b`; `left` is `a·g1` and `right` is `b·g1`; `sum` from `deps.pairing.add_g1(AddG1Params, AddG1Payload { left, right })`, over clones; outcome a `GroupOperationVector` with `input` the encodings of `left` then `right` and `output` the encoding of `sum`.

### First-group multiplication

For each index below `params.first_group.g1_mul.get()`: draws `a` then `s`; `point` is `a·g1`; `product` from `deps.pairing.mul_g1(MulG1Params, MulG1Payload { point, scalar: s })`; `input` the encoding of `point` then the encoding of `s`; `output` the encoding of `product`.

### Pairing check

For each index below `params.first_group.pairing_check.get()`: draws `a` then `b`; condition the index is even, decision `index % 2 == 0`, `t` is `deps.pairing.neg_scalar` of `deps.pairing.mul_scalar(a, b)`; otherwise draws `c` and `t` is `c`; terms `PairingProductTerm { g1: a·g1, g2: b·g2 }` and `PairingProductTerm { g1: t·g1, g2: g2 }`; `is_one` from `deps.pairing.pairing_product_is_one(PairingProductIsOneParams, PairingProductIsOnePayload { terms })` over clones; `input` the encodings of `a·g1`, `b·g2`, `t·g1`, and `g2` in that order; `output` the 32-byte word of zeros whose last byte is `0x01` where `is_one` and `0x00` otherwise; outcome a `PairingCheckVector`.

### Second group

Only with `Some(settings)`: second-group addition for each index below `settings.g2_add.get()`, the first-group addition branch over `g2`, `add_g2`, and `encode_g2`; first-group multi-scalar multiplication for each index below `settings.g1_msm.count.get()`, for each term from `0` below `settings.g1_msm.terms.get()` drawing `a_i` then `s_i`, the term `MsmG1Term { base: a_i·g1, scalar: s_i }`, `input` gaining the base's encoding then `s_i`'s encoding, then `sum` from `deps.pairing.msm_g1(MsmG1Params, MsmG1Payload { terms })` and `output` its encoding; second-group multi-scalar multiplication likewise over `settings.g2_msm`, `g2`, `MsmG2Term`, `msm_g2`, and `encode_g2`; outcome `Some(SecondGroupVectors { g2_add, g1_msm, g2_msm })`, and `None` without settings.

### Ordering

The gate, then the kinds in the order `g1_add`, `g1_mul`, `pairing_check`, `g2_add`, `g1_msm`, `g2_msm`, each record's draws in the order above; the first failed draw or sampling returns; every drawn scalar's `Secret` and every clone held in a payload is cleared when it drops; no side effect beyond the draws.

