# `rejection_vectors` — interaction spec

Branch contract for the `rejection_vectors` module of the `solidity` module of the `generate` module of `apps/harness-crypto`: `rejection_vectors` emits the decode-rejection vector records, an enumerated kind — one record per candidate encoding the pairing family defines, the outside-the-subgroup point of each source group where that group is not the whole curve and the scalar field's order as the non-canonical scalar, each naming what it decodes into and its bytes — beside the record type, the decode-target vocabulary, and the record's canonical description with its member names. Each function's branches state condition, decision, dependency call, and the exact return outcome.

## `DecodeTarget::as_str` and `DecodeTarget::try_new`

`DecodeTarget::as_str(&self) -> &'static str`: an exhaustive `match`, `G1` to `"G1"`, `G2` to `"G2"`, and `Scalar` to `"Scalar"`.

`DecodeTarget::try_new(params: DecodeTargetConstructorParams) -> DecodeTargetTryNewReturn`:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| admitted | `params.text` equals the `as_str()` of a variant | the text against each variant's name | none | `Ok` of that variant |
| unknown | `params.text` equals no variant's `as_str()` | the same comparisons | none | `Err(DecodeTargetTryNewErrorReturn::Unknown { text })`, the text moved from the params |

## `RejectionVectorDescription::try_new`

`try_new(_params: RejectionVectorDescriptionConstructorParams) -> RejectionVectorDescriptionTryNewReturn`: one branch; outcome `Ok(RejectionVectorDescription)`.

## `IEncodingContract` for `RejectionVectorDescription`

`type Described = RejectionVector`, `type FromFieldsErrorReturn = RejectionVectorFromFieldsErrorReturn`, `FIELDS` `&REJECTION_VECTOR_FIELD_KINDS`.

`to_fields(params: ToFieldsParams, payload: &RejectionVector) -> ToFieldsReturn`: one branch; outcome `Ok(ToFieldsSuccessReturn { fields: CanonicalFields { values } })`, `values` being `CanonicalFieldValue::Text` of `target.as_str()` and `CanonicalFieldValue::Bytes` of a clone of `input`, in that order.

`fields_to_value(params: FromFieldsParams, payload: CanonicalFields) -> FromFieldsReturn<RejectionVector, RejectionVectorFromFieldsErrorReturn>`:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| count | `payload.values` does not convert into `[CanonicalFieldValue; REJECTION_VECTOR_FIELD_COUNT]` | the conversion's return | none | `Err(RejectionVectorFromFieldsErrorReturn::FieldCount { expected: REJECTION_VECTOR_FIELD_COUNT, actual })`, `actual` the vector's length |
| kind | the lowest index `i` whose value is not the kind `REJECTION_VECTOR_FIELD_KINDS` holds at `i` | that index's kind against the kinds constant | none | `Err(RejectionVectorFromFieldsErrorReturn::FieldKind { index: i, expected })`, `expected` the kind `REJECTION_VECTOR_FIELD_KINDS` holds at `i` |
| target | the first field's `CanonicalFieldValue::Text` names no variant | the constructor's return | `DecodeTarget::try_new` over the first field's text | `Err(RejectionVectorFromFieldsErrorReturn::Target(error))`, the refusal unchanged |
| rebuilt | the count converts, every value holds the kind its index declares, and the text names a variant | none | `DecodeTarget::try_new` | `Ok(FromFieldsSuccessReturn { described: RejectionVector { target, input } })`, each moved from its field |

## `member_names`

`member_names(&self) -> RejectionVectorMemberNamesReturn`: for each index in order, dependency call `SolidityMemberName::try_new(SolidityMemberNameConstructorParams { text })` over `REJECTION_VECTOR_MEMBER_NAMES` at that index.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| refused | `SolidityMemberName::try_new` returns `Err(error)` at an index | the constructor's return | the constructor, once per name in order | `Err(RejectionVectorMemberNamesErrorReturn::MemberName(error))`, the refusal unchanged |
| admitted | every name is admitted | none | the constructor, once per name | `Ok` of the admitted names in order |

Every name is a distinct lowercase identifier the constructor admits, other than `line`, so no input takes the refusal, and the refusal has no unit test.

## `rejection_vectors`

`rejection_vectors<'a, P: IPairingReference>(deps: &RejectionVectorsDeps<'a, P>, params: RejectionVectorsParams, payload: RejectionVectorsPayload) -> RejectionVectorsReturn`, the trusted form: every input is a typed value from the generate concrete. The function draws no randomness, reads no clock, touches no filesystem, and decodes nothing; it names no curve or concrete; every candidate is the selected pairing's.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| first group refused | the first-group candidate search returns `Err(error)` | the call's return | `deps.pairing.g1_outside_subgroup_encoding(G1OutsideSubgroupEncodingParams, G1OutsideSubgroupEncodingPayload)` | `Err(RejectionVectorsErrorReturn::G1OutsideSubgroupEncoding(error))`, the error unchanged |
| second group refused | the second-group candidate search returns `Err(error)` | the call's return | `deps.pairing.g2_outside_subgroup_encoding(G2OutsideSubgroupEncodingParams, G2OutsideSubgroupEncodingPayload)` | `Err(RejectionVectorsErrorReturn::G2OutsideSubgroupEncoding(error))`, the error unchanged |
| computed | both searches return | none | the calls each subsection states | `Ok(RejectionVectorsSuccessReturn { records })`, the records in the order first group, second group, scalar |

### First group

Dependency call `deps.pairing.g1_outside_subgroup_encoding(G1OutsideSubgroupEncodingParams, G1OutsideSubgroupEncodingPayload)`; a refusal returns `Err(RejectionVectorsErrorReturn::G1OutsideSubgroupEncoding(error))` unchanged; condition `bytes` is `None`, no record; otherwise the record with `target` `DecodeTarget::G1` and `input` the bytes' `as_ref()` copied.

### Second group

Dependency call `deps.pairing.g2_outside_subgroup_encoding(G2OutsideSubgroupEncodingParams, G2OutsideSubgroupEncodingPayload)`; a refusal returns `Err(RejectionVectorsErrorReturn::G2OutsideSubgroupEncoding(error))` unchanged; otherwise the record with `target` `DecodeTarget::G2` and `input` the bytes' `as_ref()` copied.

### Scalar

Dependency call `deps.pairing.scalar_field_order(ScalarFieldOrderParams, ScalarFieldOrderPayload)`, unpacked irrefutably; the record with `target` `DecodeTarget::Scalar` and `input` the order's bytes.

### Ordering

The first group, then the second, then the scalar, the first refusal returned; the same pairing yields the same records; `params` and `payload` are not read; no side effect.

## Integration: callee dispositions

The `pairing/factory` entries this node's route reaches; the route of each is `create_pairing`, the arm's concrete through its `try_new`, `rejection_vectors`, and the reference method each entry names; there is no outer-edge collaborator on that route.

- `libraries_sharing_an_identifier_return_the_same_scalar_field_order` (`pairing/factory`): carried; restated where this node transforms it: the `Scalar` record's `input` is the order's bytes, equal across any two concretes whose declared target-group encoding identifiers are equal.
- `libraries_sharing_an_identifier_return_the_same_first_group_reference_encoding` (`pairing/factory`): carried; restated where this node transforms it: no `G1` record exists for `TargetGroupEncodingIdentifier::Bn254V1`, and the `G1` record's `input` is the encoding's bytes, equal across any two concretes whose identifiers are equal, for `TargetGroupEncodingIdentifier::Bls12381V1`.
- `libraries_sharing_an_identifier_return_the_same_second_group_reference_encoding` (`pairing/factory`): carried; restated where this node transforms it: the `G2` record's `input` is the encoding's bytes, equal across any two concretes whose identifiers are equal.
