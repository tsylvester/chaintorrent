# `mint_statement` — interaction spec

Branch contract for the `mint_statement` module of the `proof` crate: the mint transcript under delivery-statement version one and its canonical description, which flattens the transcript to canonical fields — each chain form through its own `ICanonicalField`, each group element through the selected pairing's precompile encoding — and rebuilds it only from fields that satisfy every kind, domain, form, and group check in ascending index. Each branch states condition, decision, dependency call, and the exact return outcome.

## `MintStatementDescription::try_new(params: MintStatementDescriptionConstructorParams<'_, P>) -> MintStatementDescriptionTryNewReturn<'_, P, F>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| construct | any params | none | none | `Ok(MintStatementDescription { pairing, forms: PhantomData })`, `pairing` the supplied `&P` stored unchanged |

The error arm has no branch: `Infallible` is uninhabited.

## `IEncodingContract` for `MintStatementDescription<'_, P, F>`

`Described` is `MintStatement<P, F>`; `FromFieldsErrorReturn` is `MintStatementFromFieldsErrorReturn<F>`; `FIELDS` is, in index order: `FixedBytes32`, `Unsigned16`, `<F::ChainIdentifier as ICanonicalField>::KIND`, `<F::Identity as ICanonicalField>::KIND`, `FixedBytes32`, `FixedBytes32`, `<F::Entitlement as ICanonicalField>::KIND`, `<F::Entitlement as ICanonicalField>::KIND`, `<F::Interval as ICanonicalField>::KIND`, `<F::Interval as ICanonicalField>::KIND`, `Unsigned16`, `<F::Identity as ICanonicalField>::KIND`, `<F::Identity as ICanonicalField>::KIND`, `Bytes` ×6, `Unsigned64`, `Bytes` ×5.

## `to_fields(&self, params: ToFieldsParams, payload: &MintStatement<P, F>) -> ToFieldsReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| encoded | any | the declared field order and kinds | index `0`: `FixedBytes32(*suite_identifier.identifier())`; `1`: `Unsigned16(suite_identifier.version())`; `2`, `3`, `6`, `7`, `8`, `9`, `11`, `12`: each form's `to_field(ToFieldParams, &form)`, the field's value moved out; `4`: `FixedBytes32(*asset_identity_hash.as_bytes())`; `5`: `FixedBytes32(*parameter_set_identifier.as_bytes())`; `10`: `Unsigned16(purpose.code())`; `13`–`18` and `20`–`24`: `self.pairing.encode_g1` over each `P::G1` field and `encode_g2` over each `P::G2` field, each encoded wrapper's `as_ref()` bytes copied into `CanonicalFieldValue::Bytes`; `19`: `Unsigned64(expiry)` | `Ok(ToFieldsSuccessReturn { fields })` with `fields.values` the 25 values in transcript order |

Fields `13`–`18` encode `buyer_keys.pk1` (G1), `buyer_keys.pk2` (G2), `envelope.c1` (G1), `envelope.c2` (G1), `envelope.d1` (G2), `envelope.d2` (G2); fields `20`–`24` encode `first_messages.hpub` (G2), `first_messages.c1` (G1), `first_messages.c2` (G1), `first_messages.d1` (G2), `first_messages.d2` (G2). Every `to_field` and `encode_*` return is `Result<_, Infallible>`; each unpack is irrefutable. `params` carries no control.

## `fields_to_value(&self, params: FromFieldsParams, payload: CanonicalFields) -> FromFieldsReturn<MintStatement<P, F>, MintStatementFromFieldsErrorReturn<F>>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| wrong count | `payload.values` does not convert into `[CanonicalFieldValue; MINT_STATEMENT_FIELD_COUNT]` | `TryFrom<Vec<CanonicalFieldValue>>` for the array, which returns the vector on failure | none | `Err(FieldCount { expected: MINT_STATEMENT_FIELD_COUNT, actual })`, `actual` the returned vector's length |
| wrong kind, directly mapped field | the count matches and the value at a directly mapped index — `0`, `1`, `4`, `5`, `10`, or `13`–`24` — is not the variant that index's kind in `FIELDS` names | the array is destructured into its fields and each directly mapped field is matched against its variant by `let … else`, in ascending index | none | `Err(FieldKind { index, expected })`, `expected` the kind at that index |
| form refused | the count matches and the form at index `2`, `3`, `6`, `7`, `8`, `9`, `11`, or `12` returns `Err(error)` from its `from_field` | that form's `from_field(FromFieldParams, value)`, the value moved in, once per form field, in ascending index | `F::ChainIdentifier::from_field` at `2`; `F::Identity::from_field` at `3`, `11`, `12`; `F::Entitlement::from_field` at `6`, `7`; `F::Interval::from_field` at `8`, `9` | `Err(ChainIdentifier { index, error })` for `2`, `Identity { index, error }` for `3`, `11`, `12`, `Entitlement { index, error }` for `6`, `7`, `Interval { index, error }` for `8`, `9`, the refusal unchanged |
| suite identifier refused | fields `0` and `1` are of their kinds and `SuiteIdentifier::try_new` over them returns `Err(error)` | the constructor runs once, after field `1` is read | `SuiteIdentifier::try_new(SuiteIdentifierConstructorParams { identifier, version })` | `Err(SuiteIdentifier(error))`, the refusal unchanged |
| asset identity hash refused | field `4` is `FixedBytes32` and the constructor returns `Err(error)` | the constructor runs once, when its field is read | `AssetIdentityHash::try_new(AssetIdentityHashConstructorParams { bytes })` | `Err(AssetIdentityHash(error))`, the refusal unchanged |
| parameter-set digest refused | field `5` is `FixedBytes32` and the constructor returns `Err(error)` | the constructor runs once, when its field is read | `ParameterSetIdentifier::try_new(ParameterSetIdentifierConstructorParams { bytes })` | `Err(ParameterSetIdentifier(error))`, the refusal unchanged |
| purpose refused | field `10` is `Unsigned16` and `MintPurpose::try_from(code)` rejects it | any code except mint or replacement fails | `MintPurpose::try_from(code)` | `Err(PurposeCode { index: 10, code })` |
| invalid group encoding | every prior check passes and a `Bytes` field at index `13`–`18` or `20`–`24` refuses to decode | decode in ascending index, first refusal decides | `self.pairing.decode_g1(DecodeG1Params, &bytes)` for `13`, `15`, `16`, `21`, `22`; `decode_g2(DecodeG2Params, &bytes)` for `14`, `17`, `18`, `20`, `23`, `24` | `Err(InvalidG1 { index, error })` or `Err(InvalidG2 { index, error })`, `error` the decoder's refusal unchanged |
| admitted | every check passes and all eleven group encodings decode | move decoded points into the typed bundles | as above | `Ok(FromFieldsSuccessReturn { described })`, `described` a `MintStatement<P, F>` holding the decoded `PublicKeysComponents`, `EnvelopeComponents`, and `MintFirstMessages` and the reconstructed `MintPurpose` |

## Ordering and invariants

- The count check precedes every field read; the fields are read in ascending index, each field's kind check, form reconstruction, or domain construction completing before the next field is read — the suite identifier is constructed once field `1` is read, the asset identity hash once `4` is read, the digest once `5`, the purpose once `10`, and group decoding begins only after every earlier check.
- The lowest failing index decides; the same payload always yields the same outcome.
- `params` carries no control and is not read.
- A reconstructed mint statement contains only group elements admitted by the selected pairing's decoders and a `MintPurpose`; a round trip preserves the exact canonical field order and values.
