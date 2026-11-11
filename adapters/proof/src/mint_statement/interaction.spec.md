# `mint_statement` — interaction spec

Branch contract for the `mint_statement` module of the `proof` crate: the mint transcript under delivery-statement version one and its canonical description, which flattens the transcript to canonical fields — each chain form through its own `ICanonicalField`, each group element through the selected pairing's precompile encoding — and rebuilds it only from fields that satisfy every kind, domain, form, and group check in ascending index. Each branch states condition, decision, dependency call, and the exact return outcome.

## `MintStatementDescription::try_new(params: MintStatementDescriptionConstructorParams<'_, P>) -> MintStatementDescriptionTryNewReturn<'_, P, F>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| construct | any params | none | none | `Ok(MintStatementDescription { pairing, forms: PhantomData })`, `pairing` the supplied `&P` stored unchanged |

The error arm has no branch: `Infallible` is uninhabited.

## `IEncodingContract` for `MintStatementDescription<'_, P, F>`

`Described` is `MintStatement<P, F>`; `FromFieldsErrorReturn` is `MintStatementFromFieldsErrorReturn`; `FIELDS` is, in index order: `FixedBytes32`, `Unsigned16`, `<F::ChainIdentifier as ICanonicalField>::KIND`, `<F::Identity as ICanonicalField>::KIND`, `FixedBytes32`, `FixedBytes32`, `<F::Entitlement as ICanonicalField>::KIND`, `<F::Entitlement as ICanonicalField>::KIND`, `<F::Interval as ICanonicalField>::KIND`, `<F::Interval as ICanonicalField>::KIND`, `Unsigned16`, `<F::Identity as ICanonicalField>::KIND`, `<F::Identity as ICanonicalField>::KIND`, `Bytes` ×6, `Unsigned64`, `Bytes` ×5.

## `to_fields(&self, params: ToFieldsParams, payload: &MintStatement<P, F>) -> ToFieldsReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| encoded | any | the declared field order and kinds | index `0`: `FixedBytes32(*suite_identifier.identifier())`; `1`: `Unsigned16(suite_identifier.version())`; `2`, `3`, `6`, `7`, `8`, `9`, `11`, `12`: each form's `to_field(ToFieldParams, &form)`, the field's value moved out; `4`: `FixedBytes32(*asset_root.as_bytes())`; `5`: `FixedBytes32(*parameter_set_identifier.as_bytes())`; `10`: `Unsigned16(purpose.code())`; `13`–`18` and `20`–`24`: `self.pairing.encode_g1` over each `P::G1` field and `encode_g2` over each `P::G2` field, each encoded wrapper's `as_ref()` bytes copied into `CanonicalFieldValue::Bytes`; `19`: `Unsigned64(expiry)` | `Ok(ToFieldsSuccessReturn { fields })` with `fields.values` the 25 values in transcript order |

Fields `13`–`18` encode `buyer_keys.pk1` (G1), `buyer_keys.pk2` (G2), `envelope.c1` (G1), `envelope.c2` (G1), `envelope.d1` (G2), `envelope.d2` (G2); fields `20`–`24` encode `first_messages.hpub` (G2), `first_messages.c1` (G1), `first_messages.c2` (G1), `first_messages.d1` (G2), `first_messages.d2` (G2). Every `to_field` and `encode_*` return is `Result<_, Infallible>`; each unpack is irrefutable. `params` carries no control.

## `fields_to_value(&self, params: FromFieldsParams, payload: CanonicalFields) -> FromFieldsReturn<MintStatement<P, F>, MintStatementFromFieldsErrorReturn>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| fields refused | `CanonicalFieldReader::try_new(payload, Self::FIELDS)` refuses the list's count, or the read at the lowest failing index refuses | the reader's `try_new`, then `read_fixed_bytes32` at `0`, `read_unsigned16` at `1`, `read_form` at `2` and `3`, `read_fixed_bytes32` at `4` and `5`, `read_form` at `6` through `9`, `read_unsigned16` at `10`, `read_form` at `11` and `12`, `read_bytes` at `13` through `18`, `read_unsigned64` at `19`, and `read_bytes` at `20` through `24`, each `read_form` typed by the form `F` names for that position, in ascending index | `CanonicalFieldReader::try_new`, then the named `read_*` calls, each once | `Err(MintStatementFromFieldsErrorReturn::Fields(error))`, `error` the reader's refusal unchanged |
| suite identifier refused | fields `0` and `1` are read and `SuiteIdentifier::try_new` over them returns `Err(error)` | the constructor runs once, after field `1` is read and before field `2` is read | `SuiteIdentifier::try_new(SuiteIdentifierConstructorParams { identifier, version })` | `Err(MintStatementFromFieldsErrorReturn::SuiteIdentifier(error))`, the refusal unchanged |
| asset root refused | field `4` is read and `Root::try_new` returns `Err(error)` | the constructor runs once, after field `4` is read and before field `5` is read | `Root::try_new(RootConstructorParams { bytes })` | `Err(MintStatementFromFieldsErrorReturn::AssetRoot(error))`, the refusal unchanged |
| parameter-set digest refused | field `5` is read and `ParameterSetIdentifier::try_new` returns `Err(error)` | the constructor runs once, after field `5` is read and before field `6` is read | `ParameterSetIdentifier::try_new(ParameterSetIdentifierConstructorParams { bytes })` | `Err(MintStatementFromFieldsErrorReturn::ParameterSetIdentifier(error))`, the refusal unchanged |
| purpose refused | field `10` is read and `MintPurpose::try_from(code)` rejects it | the conversion runs once, after field `10` is read and before field `11` is read | `MintPurpose::try_from(code)` | `Err(MintStatementFromFieldsErrorReturn::PurposeCode { index: 10, code })` |
| invalid group encoding | every field is read, every domain and purpose construction succeeds, and a `Bytes` field at index `13`–`18` or `20`–`24` refuses to decode | decode in ascending index, first refusal decides | `self.pairing.decode_g1(DecodeG1Params, &bytes)` for `13`, `15`, `16`, `21`, `22`; `decode_g2(DecodeG2Params, &bytes)` for `14`, `17`, `18`, `20`, `23`, `24` | `Err(MintStatementFromFieldsErrorReturn::InvalidG1 { index, error })` or `Err(MintStatementFromFieldsErrorReturn::InvalidG2 { index, error })`, `error` the decoder's refusal unchanged |
| admitted | every field is read, every domain and purpose construction succeeds, and all eleven group encodings decode | move decoded points into the typed bundles | as above | `Ok(FromFieldsSuccessReturn { described })`, `described` a `MintStatement<P, F>` holding the decoded `PublicKeysComponents`, `EnvelopeComponents`, and `MintFirstMessages` and the reconstructed `MintPurpose` |

## Ordering and invariants

- The reader is constructed before any field is read; the fields are read in ascending index, each field's read and any domain or purpose construction over it completing before the next field is read — the suite identifier is constructed once field `1` is read, the asset root once `4` is read, the digest once `5`, the purpose once `10`, and group decoding begins only after every field is read.
- The lowest failing index decides; the same payload always yields the same outcome.
- `params` carries no control and is not read.
- A reconstructed mint statement contains only group elements admitted by the selected pairing's decoders and a `MintPurpose`; a round trip preserves the exact canonical field order and values.

## `DeliveryPurpose::code(&self) -> u16`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| coded | the purpose | a `match` over the closed variants | none | `1` for `Mint`, `2` for `Transfer`, `3` for `Grant`, `4` for `Replacement` |

## `DeliveryPurpose::relation(&self) -> DeliveryRelation`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| related | the purpose | a `match` over the closed variants | none | `DeliveryRelation::Mint` for `Mint` and `Replacement`, `DeliveryRelation::Transfer` for `Transfer` and `Grant` |

## `TransferPurpose::code(&self) -> u16`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| coded | the purpose | a `match` over the closed variants | none | `2` for `Transfer`, `3` for `Grant` |

## `TransferPurpose::try_from(code: u16) -> Result<TransferPurpose, UndeclaredDeliveryPurposeCode>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| admitted | `code` is `2` or `3` | a `match` over the code | none | `Ok(TransferPurpose::Transfer)` for `2`, `Ok(TransferPurpose::Grant)` for `3` |
| refused | any other code, the mint relation's codes included | the same `match`'s fall-through | none | `Err(UndeclaredDeliveryPurposeCode { code })`, the code the relation refuses |

## Integration: callee dispositions

Each `pairing/factory` and `chain/create_chain_forms` entry whose route this node's description extends is carried and restated in this node's terms; no outer-edge collaborator is on any route.

- `pairing/factory` entry `each_second_group_decoder_refuses_the_second_group_reference_encoding`: carried — restated in this node's terms: a field list whose second-group buyer-key position holds the concrete's second-group reference encoding is refused by `fields_to_value` as `Err(MintStatementFromFieldsErrorReturn::InvalidG2 { index: 14, error: DecodeG2ErrorReturn::NotInSubgroup })`. Route: `create_pairing`, the arm's `try_new`, `consume_pairing`, `g2_outside_subgroup_encoding`, the chain family's `create_chain_forms`, the description's `try_new`, `fields_to_value`, and `decode_g2`.
- `pairing/factory` entry `each_first_group_decoder_refuses_the_first_group_reference_encoding_where_one_exists`: carried — restated in this node's terms: whenever the concrete's `g1_outside_subgroup_encoding` holds bytes, a field list whose first-group buyer-key position holds them is refused by `fields_to_value` as `Err(MintStatementFromFieldsErrorReturn::InvalidG1 { index: 13, error: DecodeG1ErrorReturn::NotInSubgroup })`. Route: `create_pairing`, the arm's `try_new`, `consume_pairing`, `g1_outside_subgroup_encoding`, the chain family's `create_chain_forms`, the description's `try_new`, `fields_to_value`, and `decode_g1`.
- `chain/create_chain_forms` entry `chain_forms_identity_survives_its_field_round_trip`: carried — restated in this node's terms: the identity forms at positions `3`, `11`, and `12` — the entitlement contract, the seller, and the buyer — that `to_fields` yields as fixed twenty-byte fields are rebuilt by `fields_to_value` into forms equal to the originals, over every member of `CHAIN_FORMS_CONCRETES`. Route: `create_pairing`, `consume_pairing`, `create_chain_forms`, `consume_chain_forms`, the description's `try_new`, `to_fields`, and `fields_to_value`, and the identity form's `to_field` and `from_field`.
- `chain/create_chain_forms` entry `chain_forms_entitlement_survives_its_field_round_trip`: carried — restated in this node's terms: the entitlement forms at positions `6` and `7` — the source and target entitlements — that `to_fields` yields as 256-bit fields are rebuilt by `fields_to_value` into forms equal to the originals, over every member of `CHAIN_FORMS_CONCRETES`. Route: `create_pairing`, `consume_pairing`, `create_chain_forms`, `consume_chain_forms`, the description's `try_new`, `to_fields`, and `fields_to_value`, and the entitlement form's `to_field` and `from_field`.
- `chain/create_chain_forms` entry `chain_forms_interval_survives_its_field_round_trip`: carried — restated in this node's terms: the interval forms at positions `8` and `9` — the old and new interval counters — that `to_fields` yields as 64-bit fields are rebuilt by `fields_to_value` into forms equal to the originals, over every member of `CHAIN_FORMS_CONCRETES`. Route: `create_pairing`, `consume_pairing`, `create_chain_forms`, `consume_chain_forms`, the description's `try_new`, `to_fields`, and `fields_to_value`, and the interval form's `to_field` and `from_field`.
- `chain/create_chain_forms` entry `chain_forms_chain_identifier_survives_its_field_round_trip`: carried — restated in this node's terms: the chain-identifier form at position `2` that `to_fields` yields as a 256-bit field is rebuilt by `fields_to_value` into a form equal to the original, over every member of `CHAIN_FORMS_CONCRETES`. Route: `create_pairing`, `consume_pairing`, `create_chain_forms`, `consume_chain_forms`, the description's `try_new`, `to_fields`, and `fields_to_value`, and the chain-identifier form's `to_field` and `from_field`.

## Integration: own entries

Each entry is proven by the public integration block of the same name in `tests/mint_statement_integration_test.rs`.

- `a_mint_statement_survives_the_encoding_round_trip`: condition a statement from `build_mint_statement` over the pairing the consumer received, handed with its `MintStatementDescription` to the received encoder's `encode` and the bytes returned handed to `decode` with the same description; outcome `Ok(DecodeSuccessReturn { described })` whose every field equals the original's — the forms, the domain values, the purpose, and the expiry by equality, and the group elements through the pairing's `encode_g1` and `encode_g2`; variation every member of `DECLARED_ENCODING_SELECTIONS`, each over every member of `PAIRING_CONCRETES` and of `CHAIN_FORMS_CONCRETES`, the statement's values distinct at every position so that a swap of same-kind fields fails; edge that must survive the composition: the declared field kinds are the kinds the encoder admits, and the group elements the real decoder admits are the elements the real encoder emitted.

## Integration: public surface

An outside caller invokes `run_mint_statement_case` from the crate's `mocks` surface with an `IMintStatementCaseConsumer` it writes, whose method builds the description through `MintStatementDescription::try_new` over the received pairing and observes what `to_fields`, `fields_to_value`, and the received encoder's `encode` and `decode` return; the entries proven are `a_mint_statement_survives_the_encoding_round_trip` and the carried `each_second_group_decoder_refuses_the_second_group_reference_encoding`, `each_first_group_decoder_refuses_the_first_group_reference_encoding_where_one_exists`, `chain_forms_identity_survives_its_field_round_trip`, `chain_forms_entitlement_survives_its_field_round_trip`, `chain_forms_interval_survives_its_field_round_trip`, and `chain_forms_chain_identifier_survives_its_field_round_trip`; the route of the own entry is `run_mint_statement_case`, `create_pairing`, `consume_pairing`, `create_encoding`, `consume_encoding`, `create_chain_forms`, `consume_chain_forms`, the description's `try_new` and `to_fields`, the encoder's `encode`, the encoder's `decode`, the description's `fields_to_value`, and the pairing's `decode_g1` and `decode_g2`; the route of each carried entry is the route stated in its disposition; the outer-edge collaborators are none; the fixtures are `build_mint_statement_case_deps`, `build_mint_statement_case_params`, `build_mint_statement_case_payload`, `run_mint_statement_case`, `build_mint_statement`, `invalidate_mint_statement`, `MintStatementCorruptions`, and the pairing, encoding, and chain families' official builders.

