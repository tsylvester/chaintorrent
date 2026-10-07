# rejection_vectors interactions

`rejection_vectors` enumerates the decode-rejection vector records the
Solidity pairing library's Foundry tests prove against, generic over the
selected pairing: the first group's outside-the-subgroup encoding where the
family defines one, the second group's outside-the-subgroup encoding, and the
scalar field's order as the non-canonical scalar. Each record names its decode
target and carries the candidate's bytes unchanged. The function draws no
randomness, reads no clock, touches no filesystem, and decodes nothing; it
names no curve or concrete, and every candidate is the selected pairing's.

## `DecodeTarget::as_str`

`DecodeTarget::as_str(&self) -> &'static str`: an exhaustive `match`, `G1` to
`"G1"`, `G2` to `"G2"`, and `Scalar` to `"Scalar"`.

## `DecodeTarget::try_new`

`DecodeTarget::try_new(params: DecodeTargetConstructorParams) -> DecodeTargetTryNewReturn`.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| admitted | `params.text` equals the `as_str()` of a variant | the comparison per variant | none | `Ok` of that variant |
| unknown | `params.text` equals no variant's `as_str()` | the same comparisons | none | `Err(DecodeTargetTryNewErrorReturn::Unknown { text })`, the text moved from the params |

## `RejectionVectorDescription::try_new`

`RejectionVectorDescription::try_new(_params: RejectionVectorDescriptionConstructorParams) -> RejectionVectorDescriptionTryNewReturn`:
one branch, outcome `Ok(RejectionVectorDescription)`.

## `IEncodingContract for RejectionVectorDescription`

`type Described = RejectionVector`, `type FromFieldsErrorReturn =
RejectionVectorFromFieldsErrorReturn`, `FIELDS` `&REJECTION_VECTOR_FIELD_KINDS`.

`to_fields`:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| mapped | any `RejectionVector` | none | none | `Ok(ToFieldsSuccessReturn { fields: CanonicalFields { values } })`, `values` being `CanonicalFieldValue::Text` of `target.as_str()` and `CanonicalFieldValue::Bytes` of a clone of `input`, in that order |

`fields_to_value`:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| count | `payload.values` does not convert into `[CanonicalFieldValue; REJECTION_VECTOR_FIELD_COUNT]` | the conversion | none | `Err(RejectionVectorFromFieldsErrorReturn::FieldCount { expected: REJECTION_VECTOR_FIELD_COUNT, actual })`, `actual` the vector's length |
| kind | the lowest index `i` whose value is not the kind `REJECTION_VECTOR_FIELD_KINDS` holds at `i` | the per-index kind check | none | `Err(RejectionVectorFromFieldsErrorReturn::FieldKind { index: i, expected })`, `expected` that kind |
| target | the first field's text names no decode target | the `try_new` result | `DecodeTarget::try_new` over the first field's text | `Err(RejectionVectorFromFieldsErrorReturn::Target(error))`, the refusal unchanged |
| rebuilt | every field of its kind and the target admitted | none | the same `try_new` | `Ok(FromFieldsSuccessReturn { described: RejectionVector { target, input } })`, `input` moved from the second field |

## `members`

`members(&self) -> RejectionVectorMembersReturn`: for each index in order,
dependency call
`SolidityMemberName::try_new(SolidityMemberNameConstructorParams { text })`
over `REJECTION_VECTOR_MEMBER_NAMES` at that index; a refusal returns
`Err(RejectionVectorMembersErrorReturn::MemberName(error))` unchanged; each
admitted name paired with `REJECTION_VECTOR_FIELD_KINDS` at the same index as
`SolidityRecordMember { name, kind }`; then
`SolidityRecordMembers::try_new(SolidityRecordMembersConstructorParams { members })`,
a refusal returning
`Err(RejectionVectorMembersErrorReturn::RecordMembers(error))` unchanged;
otherwise `Ok` of the members. Every name is a distinct lowercase identifier
the constructor admits, other than `line`, so no input takes either refusal,
and neither has a unit test.

## `rejection_vectors`

`rejection_vectors<'a, P: IPairingReference>(deps: &RejectionVectorsDeps<'a, P>, params: RejectionVectorsParams, payload: RejectionVectorsPayload) -> RejectionVectorsReturn`,
the trusted form: every input is a typed value from the generate concrete.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| first-group search failed | `deps.pairing.g1_outside_subgroup_encoding(G1OutsideSubgroupEncodingParams, G1OutsideSubgroupEncodingPayload)` returns `Err(error)` | the search result | `g1_outside_subgroup_encoding` | `Err(RejectionVectorsErrorReturn::G1OutsideSubgroupEncoding(error))`, the error unchanged |
| first group whole curve | the search's `bytes` is `None` | the `Option` | the same call | no record |
| first group searched | the search's `bytes` is `Some(encoding)` | the `Option` | the same call | the record with `target` `DecodeTarget::G1` and `input` the encoding's `as_ref()` copied |
| second-group search failed | `deps.pairing.g2_outside_subgroup_encoding(G2OutsideSubgroupEncodingParams, G2OutsideSubgroupEncodingPayload)` returns `Err(error)` | the search result | `g2_outside_subgroup_encoding` | `Err(RejectionVectorsErrorReturn::G2OutsideSubgroupEncoding(error))`, the error unchanged |
| second group searched | the search's `bytes` is an encoding | irrefutable | the same call | the record with `target` `DecodeTarget::G2` and `input` the encoding's `as_ref()` copied |
| scalar | always | none | `deps.pairing.scalar_field_order(ScalarFieldOrderParams, ScalarFieldOrderPayload)`, unpacked irrefutably | the record with `target` `DecodeTarget::Scalar` and `input` the order's bytes |
| computed | every search succeeded | the records assembled in the order below | the calls stated | `Ok(RejectionVectorsSuccessReturn { records })` |

### Ordering

The first group, then the second, then the scalar; the first refusal returned;
the same pairing yields the same records; `params` and `payload` are not read;
no side effect.
