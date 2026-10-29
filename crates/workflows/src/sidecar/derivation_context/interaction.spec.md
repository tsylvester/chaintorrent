# derivation_context interactions

`DerivationContext` is an owned value type: a fallible constructor `try_new` is its only producer. Its one decision is the cross-field rule between the group index and the geometry, read through `GroupIndex::value` and `PieceGeometry::group_count`; every other component arrives as an admitted instance of its own type and is held unchecked. `DerivationContextDescription` is the description that implements the encoding contract `IEncodingContract` for `DerivationContext`: it declares the context's canonical field sequence and maps a context to its canonical fields and canonical fields back to a context.

## `DerivationContext::try_new`

`DerivationContext::try_new(params: DerivationContextConstructorParams) -> DerivationContextTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| group index out of range | `params.group_index.value()` is not below `params.geometry.group_count()` | the comparison | none | `Err(DerivationContextTryNewErrorReturn::GroupIndexOutOfRange { group_index, group_count })`, the index's value and the geometry's count |
| admitted | the group index is below the group count | the same comparison | none | `Ok(DerivationContext { asset, deployment, suite, parameter_set, group_index, geometry })`, each component moved from the params |

Ordering: a single comparison decides the outcome; the same params always yield the same outcome.

## `DerivationContextDescription::try_new`

`DerivationContextDescription::try_new(params: DerivationContextDescriptionConstructorParams) -> DerivationContextDescriptionTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| constructed | none — the description takes no configuration | none | none | `Ok(DerivationContextDescription)`; the error arm has no branch |

## `DerivationContextDescription`'s `IEncodingContract`

`type Described = DerivationContext;`, `type FromFieldsErrorReturn = DerivationContextFromFieldsErrorReturn;`, `const FIELDS: &'static [CanonicalFieldKind] = &DERIVATION_CONTEXT_FIELD_KINDS;`

### `to_fields`

`to_fields(&self, params: ToFieldsParams, payload: &DerivationContext) -> ToFieldsReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| listed | none | none | none | `Ok(ToFieldsSuccessReturn { fields: CanonicalFields { values } })`, `values` in order `FixedBytes32` of the root's `as_bytes`, `FixedBytes32` of the deployment identity's `as_bytes`, `FixedBytes32` of the suite's `identifier`, `Unsigned16` of the suite's `version`, `FixedBytes32` of the parameter-set identifier's `as_bytes`, `Unsigned64` of the group index's `value`, `Unsigned32` of the geometry's `piece_size`, `Unsigned32` of its `piece_group_size`, and `Unsigned64` of its `total_extent`; `params` carries no control and is not read |

### `fields_to_value`

`fields_to_value(&self, params: FromFieldsParams, payload: CanonicalFields) -> FromFieldsReturn<DerivationContext, DerivationContextFromFieldsErrorReturn>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| wrong count | `payload.values` does not convert into `[CanonicalFieldValue; DERIVATION_CONTEXT_FIELD_COUNT]` | `TryFrom<Vec<CanonicalFieldValue>>` for the array, which returns the vector on failure | none | `Err(DerivationContextFromFieldsErrorReturn::FieldCount { expected: DERIVATION_CONTEXT_FIELD_COUNT, actual })`, `actual` the returned vector's length |
| wrong kind | the count matches and the value at some index is not of the kind `DERIVATION_CONTEXT_FIELD_KINDS` names there | the array is destructured into its nine values and each is matched against its kind in index order | none | `Err(DerivationContextFromFieldsErrorReturn::FieldKind { index, expected })` for the lowest such index, `expected` the kind named there |
| asset refused | every kind matches and `Root::try_new` over the asset's bytes returns `Err(error)` | none — the refusal is the branch | `Root::try_new` once | `Err(DerivationContextFromFieldsErrorReturn::Asset(error))`, the refusal whole |
| deployment refused | the asset is admitted and `DeploymentIdentity::try_new` returns `Err(error)` | none — the refusal is the branch | `DeploymentIdentity::try_new` once | `Err(DerivationContextFromFieldsErrorReturn::Deployment(error))`, the refusal whole |
| suite refused | the components before it are admitted and `SuiteIdentifier::try_new` over the identifier and the version returns `Err(error)` | none — the refusal is the branch | `SuiteIdentifier::try_new` once | `Err(DerivationContextFromFieldsErrorReturn::Suite(error))`, the refusal whole |
| parameter set refused | the components before it are admitted and `ParameterSetIdentifier::try_new` returns `Err(error)` | none — the refusal is the branch | `ParameterSetIdentifier::try_new` once | `Err(DerivationContextFromFieldsErrorReturn::ParameterSet(error))`, the refusal whole |
| group index | the components before it are admitted | none — every value is admitted | `GroupIndex::try_new` once, its success bound by the irrefutable pattern `let Ok(group_index) = …;`, its error arm uninhabited | the admitted `GroupIndex`, bound for the context |
| geometry refused | the components before it are admitted and `PieceGeometry::try_new` over the piece size, the piece-group size, and the total extent returns `Err(error)` | none — the refusal is the branch | `PieceGeometry::try_new` once | `Err(DerivationContextFromFieldsErrorReturn::Geometry(error))`, the refusal whole |
| context refused | every component is admitted and `DerivationContext::try_new` returns `Err(error)` | none — the refusal is the branch | `DerivationContext::try_new` once | `Err(DerivationContextFromFieldsErrorReturn::Context(error))`, the refusal whole |
| admitted | `DerivationContext::try_new` returns `Ok(described)` | none | `DerivationContext::try_new` once | `Ok(FromFieldsSuccessReturn { described })` |

Ordering: the count, then every kind in index order, then the components in field order — asset, deployment, suite, parameter set, group index, geometry — then the context's own check; the first refusal is returned; `params` carries no control and is not read.

## Invariants

- Every `DerivationContext` holds a group index below its geometry's group count; `try_new` is its only producer, and `Clone` copies only an already-admitted value, so no admitted form exists outside the constructor.
- `fields_to_value` reconstructs a context only through the components' own constructors and `DerivationContext::try_new`, so a decoded context carries the same invariant as a constructed one.

## Integration: own entries

- `every_concrete_decodes_the_derivation_context_it_encodes`: condition for each case of the encoding family's declared set, the concrete `create_encoding` hands the consumer encodes the reference context under `DerivationContextDescription` and decodes the bytes it wrote; outcome the decoded context equals the reference context; variation every case of the declared set, and a reference context whose nine canonical values are pairwise distinct within each kind, so a swapped, repeated, or dropped field changes the decoded context; edge that must survive the composition: the decoder admits the context only through this description's `fields_to_value`, whose component constructors and own check run on the decoded values.
- `every_concrete_declaring_the_ethereum_abi_identifier_encodes_the_reference_context_to_the_reference_vector`: condition for each case of the declared set whose concrete declares `EncodingIdentifier::EthereumAbiV1`, the concrete encodes the reference context under `DerivationContextDescription`; outcome the bytes equal the reference vector, nine 32-byte words: `0x01` repeated, `0x11` repeated, `0x22` repeated, `0x00…0001`, `0x33` repeated, `0x00…0007`, `0x00…4000`, `0x00…8000`, and `0x00…100000`, each integer right-aligned in its word; variation the reference context, whose piece size and piece-group size differ, so a swapped field, a wrong word width, or a misordered integer changes the bytes; edge that must survive the composition: the canonical field order this description states is the order the vector freezes, so every wrapping key derived over the encoded context is derived over one byte form.
- `every_concrete_declaring_the_ethereum_abi_identifier_decodes_the_reference_vector_to_the_reference_context`: condition for each case of the declared set whose concrete declares `EncodingIdentifier::EthereumAbiV1`, the concrete decodes the reference vector under `DerivationContextDescription`; outcome the decoded context equals the reference context; variation the same reference vector, so a field read from the wrong word yields a refusal or an unequal context; edge that must survive the composition: the concrete's canonical re-encoding of the admitted context equals the vector.
- `every_concrete_declaring_the_ethereum_abi_identifier_returns_the_context_refusal_whole`: condition for each case of the declared set whose concrete declares `EncodingIdentifier::EthereumAbiV1`, the concrete decodes the reference vector with its group-index word replaced by `0x00…0020`, thirty-two, the reference geometry's group count; outcome `Err(DecodeErrorReturn::Description(DerivationContextFromFieldsErrorReturn::Context(DerivationContextTryNewErrorReturn::GroupIndexOutOfRange { group_index: 32, group_count: 32 })))`; variation the index equal to the count, the lowest index refused, so a check against the piece count or an inclusive bound admits it; edge that must survive the composition: the context's own refusal reaches the decoder's caller whole inside the family's `Description` variant.
- `the_crate_root_refuses_a_group_index_outside_the_geometry_whole`: condition an outside caller calls `DerivationContext::try_new` through the crate barrel with the reference geometry, thirty-two groups, and the group index `32`; outcome `Err(DerivationContextTryNewErrorReturn::GroupIndexOutOfRange { group_index: 32, group_count: 32 })`, and no context; variation the index equal to the count, the lowest index refused, so an inclusive bound or a bound against the piece count admits it; edge that must survive the composition: the constructor and its refusal reach a caller outside the crate through `lib.rs`'s re-export of `provides`, the refusal whole.

## Integration: callee dispositions

No callee entry: the module's functions call only the components' own constructors — `Root::try_new`, `DeploymentIdentity::try_new`, `SuiteIdentifier::try_new`, `ParameterSetIdentifier::try_new`, `GroupIndex::try_new`, `PieceGeometry::try_new`, and `DerivationContext::try_new` — none of which declares an own entry, and `to_fields` and `fields_to_value` are callees of the encoding family's `encode` and `decode`, not callers of the factory's functions; nothing is absorbed or carried here.

## Integration: public surface

An outside caller invokes `create_encoding` from `encoding` for each case of `DECLARED_ENCODING_SELECTIONS` with deps over the official consumer mock `MockIEncodingConsumer`, carrying the description from the official description builder, the reference context from the official context builder, and the bytes the entry decodes, and observes the bytes `encoded` holds, the context `round_trip` or `decoded` holds, or the refusal `decoded` holds; and it calls `DerivationContext::try_new` through the crate barrel and observes the refusal. The entries proven are the five own entries above. The route of each encoding entry is `create_encoding`, the selected concrete's constructor, `consume_encoding`, and the concrete's `encode`, which calls `to_fields`, or its `decode`, which calls `fields_to_value`, the components' constructors, and `DerivationContext::try_new`; the route of the crate-root entry is `lib.rs`, `sidecar::derivation_context::provides`, then `DerivationContext::try_new`; the outer-edge collaborator on every encoding route is the consumer, replaced by `MockIEncodingConsumer`, and the crate-root route reaches none.
