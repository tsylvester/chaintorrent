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

## Integration: callee dispositions

Each `pairing/factory` entry whose route this node's descriptions extend is carried and restated in this node's terms; every route ends inside `fields_to_value`'s decoder call and no outer-edge collaborator is on any route.

- `pairing/factory` entry `each_first_group_decoder_refuses_the_first_group_reference_encoding_where_one_exists`: carried — restated in this node's terms: for a concrete whose `g1_outside_subgroup_encoding` holds bytes, `PossessionG1StatementDescription::fields_to_value` receives fields holding a valid key and those bytes as the commitment; outcome `Err(PossessionStatementFromFieldsErrorReturn::InvalidG1 { index: 1, error: DecodeG1ErrorReturn::NotInSubgroup })`, the concrete's refusal carried whole with the commitment's index. Route: `create_pairing`, the arm's `try_new`, `consume_pairing`, `g1_outside_subgroup_encoding`, `PossessionG1StatementDescription::try_new`, `fields_to_value`, and `decode_g1`.
- `pairing/factory` entry `each_second_group_decoder_refuses_the_second_group_reference_encoding`: carried — restated in this node's terms: for every concrete, `PossessionG2StatementDescription::fields_to_value` receives fields holding a valid key and the concrete's `g2_outside_subgroup_encoding` bytes as the commitment; outcome `Err(PossessionStatementFromFieldsErrorReturn::InvalidG2 { index: 1, error: DecodeG2ErrorReturn::NotInSubgroup })`, the concrete's refusal carried whole with the commitment's index. Route: `create_pairing`, the arm's `try_new`, `consume_pairing`, `g2_outside_subgroup_encoding`, `PossessionG2StatementDescription::try_new`, `fields_to_value`, and `decode_g2`.

## Integration: own entries

Each entry is named and proven by the public integration block of the same name in `tests/possession_statement_integration_test.rs`. Every entry runs over `PAIRING_CONCRETES` through `create_pairing` with params from `build_create_pairing_params` naming only `concrete`, so the builder's defaults admit.

- `a_first_group_statement_survives_the_round_trip_through_its_fields`: condition a `PossessionG1Statement` whose key is a concrete's first-group generator and whose commitment is that generator added to itself is handed to `to_fields`, the fields it returns are handed to `fields_to_value`, and the statement that returns is handed to `to_fields`; outcome the fields of the returned statement equal the fields the original flattened to, each in its own position; variation every concrete of the declared set, with a key different from the commitment, so a swapped field, a repeated field, or a decoder that refuses its own encoder's output fails; edge that must survive the composition: each concrete's decoder admits the canonical coordinates, curve membership, and subgroup membership of the points its own encoder wrote.
- `a_second_group_statement_survives_the_round_trip_through_its_fields`: condition a `PossessionG2Statement` whose key is a concrete's second-group generator and whose commitment is that generator added to itself is handed to `to_fields`, the fields it returns are handed to `fields_to_value`, and the statement that returns is handed to `to_fields`; outcome the fields of the returned statement equal the fields the original flattened to, each in its own position; variation every concrete of the declared set, with a key different from the commitment, so a swapped field, a repeated field, or a decoder that refuses its own encoder's output fails; edge that must survive the composition: each concrete's decoder admits the canonical coordinates, curve membership, and subgroup membership of the points its own encoder wrote.
- `libraries_sharing_an_identifier_flatten_a_first_group_statement_to_the_same_fields`: condition for every concrete, the statement of the first-group round-trip entry is handed to `to_fields`; outcome the fields returned are paired with `P::DECLARATION.target_group_encoding`, and any two concretes whose identifiers are equal return equal fields; variation every concrete of the declared set, which holds each library of each identifier; edge that must survive the composition: one byte form for the key and the commitment whichever library encoded them, so a challenge recomputed from the fields is the same on every side.
- `libraries_sharing_an_identifier_flatten_a_second_group_statement_to_the_same_fields`: condition for every concrete, the statement of the second-group round-trip entry is handed to `to_fields`; outcome the fields returned are paired with `P::DECLARATION.target_group_encoding`, and any two concretes whose identifiers are equal return equal fields; variation every concrete of the declared set, which holds each library of each identifier; edge that must survive the composition: one byte form for the key and the commitment whichever library encoded them, so a challenge recomputed from the fields is the same on every side.

## Integration: public surface

An outside caller invokes `create_pairing` from `pairing` with an `IPairingConsumer` it writes, for each concrete in `PAIRING_CONCRETES`; the consumer builds a description through `PossessionG1StatementDescription::try_new` or `PossessionG2StatementDescription::try_new` — under the `mocks` feature, through `build_possession_g1_statement_description` or `build_possession_g2_statement_description` — invokes `IEncodingContract::to_fields` and `IEncodingContract::fields_to_value` on it, and observes the fields or the refusal each returns; the entries proven are every own entry and both carried entries above; the route of each own entry is `create_pairing`, the arm's `try_new`, `consume_pairing`, the description's `try_new`, `to_fields`, `encode_g1` or `encode_g2`, and, where the entry round-trips, `fields_to_value`, `decode_g1` or `decode_g2`, and `to_fields` again; the outer-edge collaborators on every route are none; the fixtures are `build_possession_g1_statement`, `build_possession_g2_statement`, `invalidate_possession_g1_statement`, `invalidate_possession_g2_statement`, `build_possession_g1_statement_description`, `build_possession_g2_statement_description`, `build_create_pairing_params`, and `build_create_pairing_deps`.

