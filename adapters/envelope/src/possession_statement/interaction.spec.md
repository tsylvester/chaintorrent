# `possession_statement` — interaction spec

Branch contract for the `possession_statement` module of the `envelope` crate: the typed proof-of-possession statements for first- and second-group envelope keys and their canonical descriptions, which flatten each statement to the selected pairing's precompile encodings and rebuild it only from bytes those encodings decode. Each branch states condition, decision, dependency call, and the exact return outcome. `PossessionG1StatementDescription` and `PossessionG2StatementDescription` are identical in shape; where the two differ, the G1 member uses `encode_g1`/`decode_g1`, `P::G1`, and `PossessionStatementFromFieldsErrorReturn::InvalidG1`, and the G2 member uses `encode_g2`/`decode_g2`, `P::G2`, and `InvalidG2`.

## `PossessionG1StatementDescription::try_new(params: PossessionG1StatementDescriptionConstructorParams<'_, P>) -> PossessionG1StatementDescriptionTryNewReturn<'_, P>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| construct | any params | none | none | `Ok(PossessionG1StatementDescription { pairing })`, `pairing` the supplied `&P` stored unchanged |

`PossessionG2StatementDescription::try_new` is the same over its params and return. The error arm has no branch: `Infallible` is uninhabited.

## `IEncodingContract` for the descriptions

Each description implements `IEncodingContract` with `Described` its matching `PossessionG1Statement<P>` or `PossessionG2Statement<P>`, `FromFieldsErrorReturn` the shared `PossessionStatementFromFieldsErrorReturn`, and `FIELDS` the shared `POSSESSION_STATEMENT_FIELD_KINDS` — `Bytes` for the key's encoding, then `Bytes` for the commitment's encoding.

## `to_fields(&self, params: ToFieldsParams, payload: &Described) -> ToFieldsReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| encoded | any | key's encoding first, commitment's second | `self.pairing.encode_g1(EncodeG1Params, EncodeG1Payload { point: payload.key.clone() })`, then the same over `payload.commitment`; each encoded wrapper's `as_ref()` bytes copied into `CanonicalFieldValue::Bytes` | `Ok(ToFieldsSuccessReturn { fields })` with `fields.values` the two `Bytes` values in statement order |

The `encode_g1`/`encode_g2` returns are `Result<_, Infallible>`; the error arm has no branch and each unpack is irrefutable. `params` carries no control. The G2 member calls `encode_g2` throughout.

## `fields_to_value(&self, params: FromFieldsParams, payload: CanonicalFields) -> FromFieldsReturn<Described, PossessionStatementFromFieldsErrorReturn>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| wrong count | `payload.values` does not convert into `[CanonicalFieldValue; POSSESSION_STATEMENT_FIELD_COUNT]` | `TryFrom<Vec<CanonicalFieldValue>>` for the array, which returns the vector on failure | none | `Err(PossessionStatementFromFieldsErrorReturn::FieldCount { expected: POSSESSION_STATEMENT_FIELD_COUNT, actual })`, `actual` the returned vector's length |
| wrong kind | the count matches and the value at some index is not `CanonicalFieldValue::Bytes` | the array is destructured into its fields and each is matched against `CanonicalFieldValue::Bytes`, index `0` then index `1` | none | `Err(PossessionStatementFromFieldsErrorReturn::FieldKind { index, expected: CanonicalFieldKind::Bytes })` for the lowest such index |
| invalid key encoding | count and kinds pass; the key's bytes refuse to decode | `decode_g1`/`decode_g2` on the key's bytes | `self.pairing.decode_g1(DecodeG1Params, &bytes)` | `Err(PossessionStatementFromFieldsErrorReturn::InvalidG1 { index: 0, error })`, `error` the decoder's refusal unchanged |
| invalid commitment encoding | count and kinds pass; the key decodes and the commitment's bytes refuse to decode | `decode_g1`/`decode_g2` on the commitment's bytes | `self.pairing.decode_g1(DecodeG1Params, &bytes)` | `Err(PossessionStatementFromFieldsErrorReturn::InvalidG1 { index: 1, error })`, `error` the decoder's refusal unchanged |
| admitted | every check passes and both elements decode | none further | as above | `Ok(FromFieldsSuccessReturn { described })`, the decoded `P::G1` key and commitment moved into `PossessionG1Statement<P>` |

The G2 member calls `decode_g2` throughout and wraps decoder refusals in `InvalidG2` at the same indices.

## Ordering and invariants

- `fields_to_value` decides in the stated order: count, then kinds in index order, then group decoding in index order. No kind check is skipped by an earlier one passing; no decode runs before every kind check.
- `params` carries no control and is not read in either method.
- Only a pair of successfully decoded group elements of the selected pairing, in the same group the description describes, can enter a trusted `PossessionG1Statement<P>` or `PossessionG2Statement<P>`; no path admits raw bytes into a statement.

