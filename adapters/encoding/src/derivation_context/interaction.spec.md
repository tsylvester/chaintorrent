# derivation_context interactions

`DerivationContextDescription` is the encoding family's description of `DerivationContext`. Its fallible constructor `try_new` is its only producer, and its `IEncodingContract` implementation states the context's canonical field sequence once, format-free: `Described` is `DerivationContext`, `FromFieldsErrorReturn` is `DerivationContextFromFieldsErrorReturn`, and `FIELDS` is `&DERIVATION_CONTEXT_FIELD_KINDS`.

## `DerivationContextDescription::try_new`

`DerivationContextDescription::try_new(params: DerivationContextDescriptionConstructorParams) -> DerivationContextDescriptionTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| admitted | any params | none | none | `Ok(DerivationContextDescription)`; the error arm has no branch |

## `IEncodingContract::to_fields`

`DerivationContextDescription::to_fields(params: ToFieldsParams, payload: &DerivationContext) -> ToFieldsReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| admitted | any admitted context | none | the context's six accessors and each component's accessors, once each | `Ok(ToFieldsSuccessReturn { fields: CanonicalFields { values } })`, `values` being `Text` of the asset name, `Text` of the asset version — each copied into a `String` — `FixedBytes32` of the deployment identity's bytes, `FixedBytes32` of the suite identifier, `Unsigned16` of the suite version, `FixedBytes32` of the parameter-set identifier's bytes, `Unsigned64` of the group index, `Unsigned32` of the piece size, `Unsigned32` of the piece-group size, and `Unsigned64` of the total extent, in that order |

`params` carries no control and is not read.

## `IEncodingContract::fields_to_value`

`DerivationContextDescription::fields_to_value(params: FromFieldsParams, payload: CanonicalFields) -> FromFieldsReturn<DerivationContext, DerivationContextFromFieldsErrorReturn>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| wrong count | `payload.values` does not convert into `[CanonicalFieldValue; DERIVATION_CONTEXT_FIELD_COUNT]` | `TryFrom<Vec<CanonicalFieldValue>>` for the array, which returns the vector on failure | none | `Err(DerivationContextFromFieldsErrorReturn::FieldCount { expected: DERIVATION_CONTEXT_FIELD_COUNT, actual })`, `actual` the returned vector's length |
| wrong kind | the count matches and the value at some index is not the variant `DERIVATION_CONTEXT_FIELD_KINDS` names at that index | the array is destructured into its fields and each is matched against its expected variant, index `0` through `9` in ascending order, before any constructor is called | none | `Err(DerivationContextFromFieldsErrorReturn::FieldKind { index, expected })` for the lowest such index, `expected` the kind `DERIVATION_CONTEXT_FIELD_KINDS` names there |
| component refused | every kind matches and a component's constructor refuses | the constructors are called in field order — `AssetIdentity::try_new` over the asset name and version, `DeploymentIdentity::try_new`, `SuiteIdentifier::try_new` over the suite identifier and version, `ParameterSetIdentifier::try_new`, `GroupIndex::try_new` unpacked irrefutably, and `PieceGeometry::try_new` over the piece size, piece-group size, and total extent | each constructor at most once, none after the first refusal | `Err` holding the first refusal unchanged in its variant: `AssetIdentity`, `DeploymentIdentity`, `SuiteIdentifier`, `ParameterSetIdentifier`, or `PieceGeometry` |
| context refused | every component is admitted and `DerivationContext::try_new` refuses | the context constructor's result | `DerivationContext::try_new` once over the six components | `Err(DerivationContextFromFieldsErrorReturn::DerivationContext(error))`, the refusal unchanged |
| admitted | every check and constructor passes | the same | the same constructors | `Ok(FromFieldsSuccessReturn { described })` holding the constructed context |

Ordering: the count precedes every kind, every kind precedes every constructor, the constructors run in field order, and the context constructor runs last; the same payload always yields the same outcome. `params` carries no control and is not read.

## Invariants

- The kinds of the values `to_fields` returns are `DERIVATION_CONTEXT_FIELD_KINDS` in order, and `fields_to_value` admits exactly that sequence.

## Integration: own entries

Each entry is named and proven by the public integration block of the same name.

- `a_context_survives_the_round_trip_through_its_canonical_fields`: condition an admitted `DerivationContext` is handed to `to_fields` and the fields it returns are handed to `fields_to_value`; outcome `Ok(FromFieldsSuccessReturn { described })` with `described` equal to the original context; variation a context whose same-kind fields hold distinct values, among the text values, among the 32-byte values, among the 32-bit values, and among the 64-bit values, whose 16-bit suite version is at its maximum, whose piece-group size exceeds the `i32` range, and whose group index and total extent exceed the 32-bit range, so a swap of same-kind fields, a dropped field, or a narrowed width yields a refusal or an unequal context; edge that must survive the composition: the context constructor's group-index rule admits the rebuilt context at the full width of the extent, so the fields of an admitted context are never refused on the way back.

## Integration: public surface

An outside caller invokes `DerivationContextDescription::try_new` — under the `mocks` feature, through the official mock builder `build_derivation_context_description` — `IEncodingContract::to_fields`, and `IEncodingContract::fields_to_value`, and observes the fields, the context, or the refusal each returns. The entry proven is `a_context_survives_the_round_trip_through_its_canonical_fields` above. Its route is `to_fields` then `fields_to_value`, each running `domain`'s accessors and constructors as themselves; the outer-edge collaborators on that route are none, `domain` being a pure value library, and the fixtures are `build_derivation_context_description`, `build_to_fields_params`, and `build_from_fields_params` of this crate's mocks and `build_derivation_context`, `build_asset_identity`, `build_deployment_identity`, `build_suite_identifier`, `build_parameter_set_identifier`, `build_group_index`, and `build_piece_geometry` of `domain`'s mocks.

