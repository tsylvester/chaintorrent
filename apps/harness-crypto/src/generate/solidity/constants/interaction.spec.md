# `constants` — interaction spec

Branch contract for the `constants` module of the `solidity` module of the `generate` module of `apps/harness-crypto`: `constants` emits the constants library's entries — the identity mapping's, the proofs of possession's, and the delivery proof's challenge and weight domain tags read from their families' declarations, the delivery-statement version, the delivery purposes' codes, and the mint and transfer transcripts' field sequences as ABI type names — and the cell's values record — the curve's and the precompile encoding's declared names, the scalar field's order, the generators' precompile encodings, and whether the verifier has second-group arithmetic — generic over the selected pairing and the chain's forms, beside the record's canonical description with its member names. Each function's branches state condition, decision, dependency call, and the exact return outcome.

## `CellValuesDescription::try_new`

`try_new(_params: CellValuesDescriptionConstructorParams) -> CellValuesDescriptionTryNewReturn`: one branch; outcome `Ok(CellValuesDescription)`.

## `IEncodingContract` for `CellValuesDescription`

`type Described = CellValues`, `type FromFieldsErrorReturn = CellValuesFromFieldsErrorReturn`, `FIELDS` `&CELL_VALUES_FIELD_KINDS`.

`to_fields(params: ToFieldsParams, payload: &CellValues) -> ToFieldsReturn`: one branch; outcome `Ok(ToFieldsSuccessReturn { fields: CanonicalFields { values } })`, `values` being `CanonicalFieldValue::Text` of clones of `curve_name` and `precompile_encoding`, `CanonicalFieldValue::FixedBytes32` of `scalar_field_order`, `CanonicalFieldValue::Bytes` of clones of `g1_generator` and `g2_generator`, and `CanonicalFieldValue::Unsigned16` of `1` where `second_group_arithmetic` holds and `0` otherwise, in that order.

`fields_to_value(params: FromFieldsParams, payload: CanonicalFields) -> FromFieldsReturn<CellValues, CellValuesFromFieldsErrorReturn>`:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| fields refused | `CanonicalFieldReader::try_new` or any `read_*` returns `Err(error)` | the call's return | `CanonicalFieldReader::try_new(payload, Self::FIELDS)`, then `read_text` for `curve_name` and `precompile_encoding`, `read_fixed_bytes32` for `scalar_field_order`, `read_bytes` for `g1_generator` and `g2_generator`, and `read_unsigned16` for `second_group_arithmetic`, in that order | `Err(CellValuesFromFieldsErrorReturn::Fields(error))`, the refusal unchanged |
| second-group arithmetic | the `u16` `read_unsigned16` returned is not `0` or `1` | a `match` on the read value, `0` to `false` and `1` to `true` | the same read | `Err(CellValuesFromFieldsErrorReturn::SecondGroupArithmetic { actual })`, `actual` the read value |
| rebuilt | the reader's construction, every read, and the match return `Ok` | none | the same calls | `Ok(FromFieldsSuccessReturn { described })`, each member the value its read returned |

## `member_names`

`member_names(&self) -> CellValuesMemberNamesReturn`: for each index in order, dependency call `SolidityMemberName::try_new(SolidityMemberNameConstructorParams { text })` over `CELL_VALUES_MEMBER_NAMES` at that index.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| refused | `SolidityMemberName::try_new` returns `Err(error)` at an index | the constructor's return | the constructor, once per name in order | `Err(CellValuesMemberNamesErrorReturn::MemberName(error))`, the refusal unchanged |
| admitted | every name is admitted | none | the constructor, once per name | `Ok` of the admitted names in order |

Every name is a lowercase identifier the constructor admits, other than `line`, so no input takes the refusal.

## `constants`

`constants<'a, P: IPairingReference, E: IEncoderAdapter, F: IChainForms>(deps: &ConstantsDeps<'a, P, E>, params: ConstantsParams, payload: ConstantsPayload<'_>) -> ConstantsReturn`, the trusted form: every input is a typed value from the generate concrete. The function reads no clock, draws no randomness, and touches no filesystem; it names no curve, concrete, or chain; every tag, version, code, and kind it emits is read from its producer. It holds no table from a kind to a type name and no table of its own: each name is an uppercase identifier `SolidityConstantName::try_new` admits, every type name and `,` is printable ASCII `SolidityStringLiteral::try_new` admits, and each pairing concrete returns its order as 32 big-endian bytes.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| unsupported version | `params.statement_version` is not `DELIVERY_STATEMENT_VERSION_ONE` | equality, before any other step | none | `Err(ConstantsErrorReturn::UnsupportedStatementVersion { version: params.statement_version })` |
| constant name refused | `SolidityConstantName::try_new(SolidityConstantNameConstructorParams { text })` returns `Err(error)` for an entry's name | the constructor's return | `SolidityConstantName::try_new`, once per entry | `Err(ConstantsErrorReturn::ConstantName(error))`, the refusal unchanged |
| ABI type refused | `SolidityAbiType::try_new(SolidityAbiTypeConstructorParams { name })` returns `Err(error)` for a kind's type name | the constructor's return | `deps.encoder.type_name(TypeNameParams, kind)` then the constructor, once per kind | `Err(ConstantsErrorReturn::AbiType(error))`, the refusal unchanged |
| field sequence refused | `SolidityStringLiteral::try_new(SolidityStringLiteralConstructorParams { text })` returns `Err(error)` for a sequence | the constructor's return | the constructor, once per sequence | `Err(ConstantsErrorReturn::StringLiteral(error))`, the refusal unchanged |
| order length | `<[u8; 32]>::try_from(success.bytes)` returns the vector | the conversion's return | `deps.pairing.scalar_field_order(ScalarFieldOrderParams, ScalarFieldOrderPayload)`, its success unpacked irrefutably | `Err(ConstantsErrorReturn::ScalarFieldOrderLength { actual })`, `actual` the vector's length |
| computed | none of the above | none | the calls each subsection states | `Ok(ConstantsSuccessReturn { entries, cell_values })`, the entries and the record in the order `Computed` states |

### The field sequences

For each of `<MintStatementDescription<'a, P, F> as IEncodingContract>::FIELDS` and `<TransferStatementDescription<'a, P, F> as IEncodingContract>::FIELDS`: each kind's type name is `deps.encoder.type_name(TypeNameParams, kind)`, its success unpacked irrefutably, wrapped by `SolidityAbiType::try_new(SolidityAbiTypeConstructorParams { name })` over the name's `as_str()` text, once per kind in field order, a refusal returned before any later kind is read; the admitted types' `as_str()` texts are joined in field order by `,`, and the joined text is wrapped by `SolidityStringLiteral::try_new`, once per sequence.

### The scalar field order

`deps.pairing.scalar_field_order(ScalarFieldOrderParams, ScalarFieldOrderPayload)`, its success unpacked irrefutably; the array `<[u8; 32]>::try_from(success.bytes)` admits is the record's `scalar_field_order`; each pairing concrete returns its order as 32 big-endian bytes.

### The generators' encodings

`deps.pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload)`, then `deps.pairing.encode_g1(EncodeG1Params, EncodeG1Payload { point })`, then `g2_generator` and `encode_g2` likewise, each success unpacked irrefutably; each encoding's `as_ref()` bytes moved into the record's `g1_generator` and `g2_generator`.

### The second-group arithmetic

An exhaustive `match` on `P::DECLARATION.verifier_group_arithmetic`, `BothGroups` to `true` and `FirstGroupOnly` to `false`, the record's `second_group_arithmetic`.

### The purposes' entries

For each `purpose` of `DELIVERY_PURPOSES` in order: the name by an exhaustive `match`, `DeliveryPurpose::Mint` to `PURPOSE_MINT`, `Transfer` to `PURPOSE_TRANSFER`, `Grant` to `PURPOSE_GRANT`, and `Replacement` to `PURPOSE_REPLACEMENT`, and the value `SolidityConstantValue::Uint16(purpose.code())`.

### The record's names

The record's `curve_name` is `format!("{:?}", P::DECLARATION.curve)` and its `precompile_encoding` is `format!("{:?}", P::DECLARATION.precompile_encoding)`, the `Debug` text that names the encoding the pairing declares.

### Computed

`Ok(ConstantsSuccessReturn { entries, cell_values })`, the entries in this order: `IDENTITY_TAG`, `Bytes` of `payload.kem.identity_tag`; `POSSESSION_G1_TAG` and `POSSESSION_G2_TAG`, `Bytes` of `payload.key_agreement.possession_g1_tag` and `possession_g2_tag`; `CHALLENGE_TAG` and `WEIGHT_TAG`, `Bytes` of `payload.delivery_proof.challenge_tag` and `weight_tag`; `DELIVERY_STATEMENT_VERSION`, `Uint16` of `params.statement_version`; the purposes' entries; and `MINT_FIELDS` and `TRANSFER_FIELDS`, `String` of the mint and the transfer field sequences; and `cell_values` the record the branches above assembled.

### Ordering

The version check, then the branches in the order the table states them with the first refusal returned; the entries assembled in the order `Computed` states; no side effect.

## Integration: callee dispositions

The `chain/create_chain_forms` entries and the `pairing/factory` entry this node's route reaches.

- `chain_forms_identity_survives_its_field_round_trip` (`chain/create_chain_forms`): absorbed for its `KIND` outcome; proven by `the_entries_carry_the_mint_and_transfer_field_sequences_over_the_chain_forms` in `integration_test.rs`. Restated in this node's terms: the kind the identity form reports is the kind whose type name the encoder returns at the identity's positions in `MINT_FIELDS` and `TRANSFER_FIELDS`.
- `chain_forms_entitlement_survives_its_field_round_trip` (`chain/create_chain_forms`): absorbed for its `KIND` outcome; proven by the same block. Restated: the kind the entitlement form reports is the kind whose type name the encoder returns at the entitlement's positions in the field sequences.
- `chain_forms_interval_survives_its_field_round_trip` (`chain/create_chain_forms`): absorbed for its `KIND` outcome; proven by the same block. Restated: the kind the interval form reports is the kind whose type name the encoder returns at the interval's positions in the field sequences.
- `chain_forms_chain_identifier_survives_its_field_round_trip` (`chain/create_chain_forms`): absorbed for its `KIND` outcome; proven by the same block. Restated: the kind the chain identifier form reports is the kind whose type name the encoder returns at the chain identifier's positions in the field sequences.
- `libraries_sharing_an_identifier_return_the_same_scalar_field_order` (`pairing/factory`): absorbed; proven by `libraries_sharing_an_identifier_carry_the_same_scalar_field_order` in `integration_test.rs`. Restated in this node's terms: the record's `scalar_field_order` member holds the order's 32 big-endian bytes, equal across any two concretes whose declared target-group encoding identifiers are equal.

## Integration: own entries

Each entry is proven by the block of the private integration test element `integration_test.rs` of the same name. Every entry runs `constants` at `DELIVERY_STATEMENT_VERSION_ONE` over every member of `PAIRING_CONCRETES` and every member of `CHAIN_FORMS_CONCRETES`, each over the ABI concrete the encoding family's declared set holds, with the payload built from the official declaration builders, reads each entry from the returned entries by its constant name, and reads each record member from the returned `cell_values`.

- `the_entries_carry_each_declarations_domain_tag_under_its_own_name`: condition the KEM, key-agreement, and delivery-proof declarations are built carrying the tags each family's real concrete declares, as `kem/bb1_depth_one`, `envelope/pairing_elgamal`, and `proof/schnorr_fs` state them; outcome `Ok` whose entries `IDENTITY_TAG`, `POSSESSION_G1_TAG`, `POSSESSION_G2_TAG`, `CHALLENGE_TAG`, and `WEIGHT_TAG` are `Bytes` holding the KEM declaration's `identity_tag`, the key agreement's `possession_g1_tag` and `possession_g2_tag`, and the delivery proof's `challenge_tag` and `weight_tag`, each equal to the literal its owning node states for that tag; variation every concrete of both declared sets, the tags differing from each other and from every builder default, so that an entry read from another field, from a builder default, or under another name fails; edge that must survive the composition: each tag is read from the declaration the payload carries and `constants` holds none.
- `the_entries_carry_the_statement_version_and_each_purpose_code_in_declared_order`: condition the params carry `DELIVERY_STATEMENT_VERSION_ONE`; outcome `Ok` whose entry `DELIVERY_STATEMENT_VERSION` is `Uint16` holding the value of `DELIVERY_STATEMENT_VERSION_ONE` and whose entries `PURPOSE_MINT`, `PURPOSE_TRANSFER`, `PURPOSE_GRANT`, and `PURPOSE_REPLACEMENT` are `Uint16` holding the codes `DeliveryPurpose::Mint`, `Transfer`, `Grant`, and `Replacement` declare, each the literal `proof/mint_statement` states, the purposes' entries following `DELIVERY_PURPOSES` order; variation every concrete of both declared sets, the codes differing from each other so that swapped codes or a reordering fail; edge that must survive the composition: the codes are the purposes' own and the entry names are the function's.
- `the_entries_carry_the_mint_and_transfer_field_sequences_over_the_chain_forms`: condition the forms the chain family hands the consumer are the concrete's; outcome `Ok` whose entries `MINT_FIELDS` and `TRANSFER_FIELDS` are `String` holding the type names the ABI concrete returns for the kinds of `MintStatementDescription::FIELDS` and `TransferStatementDescription::FIELDS` joined by `,` in field order, each equal to the literal the block assembles from the field kinds the description's interaction spec lists, the forms' kinds in the `context_slice`, and the ABI table `encoding/abi` states; variation every concrete of both declared sets, each sequence compared with its own literal; edge that must survive the composition: a fixed twenty-byte kind is `bytes20`, never `address`.
- `the_record_carries_the_scalar_field_order_as_32_big_endian_bytes`: condition the pairing handed is a concrete of `PAIRING_CONCRETES`; outcome `Ok` whose member `cell_values.scalar_field_order` holds the order's 32 big-endian bytes the same pairing's `decode_scalar` refuses with `NonCanonical` and whose bytes less one it admits; variation every concrete of `PAIRING_CONCRETES`, so a constant, a truncated, a little-endian, or an off-by-one order fails; edge that must survive the composition: the order is read through the family's reference trait and checked through its arithmetic trait.
- `the_record_carries_the_generators_precompile_encodings`: condition the pairing handed is a concrete of `PAIRING_CONCRETES`; outcome `Ok` whose members `cell_values.g1_generator` and `cell_values.g2_generator` are bytes the same pairing's `decode_g1` and `decode_g2` admit, each decoded point's sum with the negation of that group's generator being the identity; variation every concrete of `PAIRING_CONCRETES`, so a member holding the other group's generator or another point fails at the decoder; edge that must survive the composition: the bytes are the encoding the pairing's decoders read.
- `the_record_carries_the_verifier_form_the_pairing_declares`: condition the pairing handed is a concrete of `PAIRING_CONCRETES`; outcome `Ok` whose member `cell_values.second_group_arithmetic` is `false` for a pairing whose declaration carries `PrecompileEncoding::Eip196Eip197`, which has no second-group precompile, and `true` for `PrecompileEncoding::Eip2537`, which has both groups, the expectation keyed on the precompile encoding and never on the `verifier_group_arithmetic` field the function reads; variation every concrete of `PAIRING_CONCRETES`, which holds a pairing of each encoding, so a constant fails; edge that must survive the composition: the form is read from the declaration of the pairing handed.
- `the_entries_are_listed_in_declared_order`: condition `constants` is called over a concrete of each declared set; outcome `Ok` whose entry names, in order, are `IDENTITY_TAG`, `POSSESSION_G1_TAG`, `POSSESSION_G2_TAG`, `CHALLENGE_TAG`, `WEIGHT_TAG`, `DELIVERY_STATEMENT_VERSION`, `PURPOSE_MINT`, `PURPOSE_TRANSFER`, `PURPOSE_GRANT`, `PURPOSE_REPLACEMENT`, `MINT_FIELDS`, and `TRANSFER_FIELDS`; variation every concrete of both declared sets; edge that must survive the composition: the order is the function's and not the declarations'.

## Integration: private surface

The chain of real functions `run_constants_case`, `create_pairing`, the arm's concrete through its `try_new`, `consume_pairing`, `create_encoding`, the ABI concrete, `consume_encoding`, `create_chain_forms`, the concrete's producer, `consume_chain_forms`, `constants`, the encoder's `type_name`, `SolidityConstantName::try_new`, `SolidityStringLiteral::try_new`, `SolidityAbiType::try_new` and `as_str`, and the pairing methods the blocks call as the oracle; no outer-edge collaborator is mocked, the declarations being builder data and the function reading no clock, drawing no randomness, and touching no filesystem; the observable result is the `ConstantsReturn` and the pairing's decoder and group-operation results; the entries proven are every own entry above and the absorbed entry.
