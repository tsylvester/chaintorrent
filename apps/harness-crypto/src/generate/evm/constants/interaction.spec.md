# constants interactions

`constants` reads each value the Solidity constants library must carry — the identity mapping's, the proofs of possession's, and the delivery proof's domain tags, the delivery-statement version, the delivery purposes' codes, the mint and transfer transcripts' field sequences as ABI type names, the scalar field's order, the generators' precompile encodings, and whether the verifier has second-group arithmetic — from the pairing, the declarations, and the transcript descriptions it is handed, and returns them as named `SolidityLibraryEntry` values. It reads no clock, draws no randomness, and touches no filesystem; every tag, version, code, and kind it emits is read from its producer.

## `constants`

`constants<'a, P: IPairingReference, F: IChainForms>(deps: &ConstantsDeps<'a, P>, params: ConstantsParams, payload: ConstantsPayload<'_>) -> ConstantsReturn`, the trusted form: every input is a typed value from the generate concrete.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| unsupported version | `params.statement_version` is not `DELIVERY_STATEMENT_VERSION_ONE` | equality, before any other step | none | `Err(ConstantsErrorReturn::UnsupportedStatementVersion { version: params.statement_version })` |
| constant name refused | `SolidityConstantName::try_new(SolidityConstantNameConstructorParams { text })` returns `Err(error)` for an entry's name | the constructor's refusal | that constructor, once per entry | `Err(ConstantsErrorReturn::ConstantName(error))`, the refusal unchanged |
| string literal refused | `SolidityStringLiteral::try_new(SolidityStringLiteralConstructorParams { text })` returns `Err(error)` for a field sequence | the constructor's refusal | that constructor, once per sequence | `Err(ConstantsErrorReturn::StringLiteral(error))`, the refusal unchanged |
| scalar field order length | `<[u8; 32]>::try_from(success.bytes)` returns the vector | the order's length is not 32 bytes | `deps.pairing.scalar_field_order(ScalarFieldOrderParams, ScalarFieldOrderPayload)`, its success unpacked irrefutably | `Err(ConstantsErrorReturn::ScalarFieldOrderLength { actual })`, `actual` the vector's length |
| computed | no branch above | the entries assembled in the order below | the constructor and pairing calls stated | `Ok(ConstantsSuccessReturn { entries })` |

Every name the function holds is an uppercase identifier `SolidityConstantName` admits, so no input takes the constant-name branch and it has no unit test; every ABI type name and `,` is printable ASCII `SolidityStringLiteral` admits, so no input takes the string-literal branch and it has no unit test; each pairing concrete returns its order as 32 big-endian bytes, so no input takes the order-length branch and it has no unit test.

### Field sequences

For each of `<MintStatementDescription<'a, P, F> as IEncodingContract>::FIELDS` and `<TransferStatementDescription<'a, P, F> as IEncodingContract>::FIELDS`, each kind's ABI type name is `SolidityAbiType::try_new(SolidityAbiTypeConstructorParams { kind })`, unpacked irrefutably, read through `as_str()`, the names joined in field order by `,`; the joined text is `SolidityStringLiteral::try_new(SolidityStringLiteralConstructorParams { text })`, once per sequence, a refusal taking the string-literal branch.

### Scalar field order

`deps.pairing.scalar_field_order(ScalarFieldOrderParams, ScalarFieldOrderPayload)`, its success unpacked irrefutably; `<[u8; 32]>::try_from(success.bytes)` returning the vector takes the scalar-field-order-length branch; otherwise the array is the `big_endian` of `SolidityUint256::try_new(SolidityUint256ConstructorParams { big_endian })`, unpacked irrefutably.

### Generators

`deps.pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload)`, then `deps.pairing.encode_g1(EncodeG1Params, EncodeG1Payload { point })`, then `g2_generator` and `encode_g2` likewise, each success unpacked irrefutably; each encoding's `as_ref()` bytes copied into `SolidityConstantValue::Bytes`.

### Second-group arithmetic

An exhaustive `match` on `P::DECLARATION.verifier_group_arithmetic`, `VerifierGroupArithmetic::BothGroups` to `true` and `VerifierGroupArithmetic::FirstGroupOnly` to `false`.

### Purposes

For each `purpose` of `DELIVERY_PURPOSES` in order, the entry's name by an exhaustive `match` — `DeliveryPurpose::Mint` to `PURPOSE_MINT`, `DeliveryPurpose::Transfer` to `PURPOSE_TRANSFER`, `DeliveryPurpose::Grant` to `PURPOSE_GRANT`, and `DeliveryPurpose::Replacement` to `PURPOSE_REPLACEMENT` — and the value `SolidityConstantValue::Uint16(purpose.code())`.

### Computed entries

In this order: `IDENTITY_TAG`, `SolidityConstantValue::Bytes` of `payload.kem.identity_tag`; `POSSESSION_G1_TAG` and `POSSESSION_G2_TAG`, `Bytes` of `payload.key_agreement.possession_g1_tag` and `possession_g2_tag`; `CHALLENGE_TAG` and `WEIGHT_TAG`, `Bytes` of `payload.delivery_proof.challenge_tag` and `weight_tag`; `DELIVERY_STATEMENT_VERSION`, `Uint16` of `params.statement_version`; the purposes' entries; `MINT_FIELDS` and `TRANSFER_FIELDS`, `SolidityConstantValue::String` of the mint and the transfer field sequences; `SCALAR_FIELD_ORDER`, `Uint256` of the order; `G1_GENERATOR` and `G2_GENERATOR`, `Bytes` of the generators' encodings; and `SECOND_GROUP_ARITHMETIC`, `Bool` of the second-group arithmetic.

Ordering: the version check, then the entries in the order above, the first refusal returned; the same deps, params, and payload yield the same entries; no side effect.

## Integration: callee dispositions

The entries of `chain/create_chain_forms` and `pairing/factory` that this node's routes reach, each dispositioned as stated.

- `chain_forms_identity_survives_its_field_round_trip` (`chain/create_chain_forms`): absorbed for its `KIND` outcome; proven by the `integration_test.rs` block for `the_entries_carry_the_mint_and_transfer_field_sequences_over_the_chain_forms`, restated in this node's terms as: the kind the identity form reports is the kind whose ABI type name stands at the identity's positions in `MINT_FIELDS` and `TRANSFER_FIELDS`.
- `chain_forms_entitlement_survives_its_field_round_trip` (`chain/create_chain_forms`): absorbed for its `KIND` outcome; proven by the `integration_test.rs` block for `the_entries_carry_the_mint_and_transfer_field_sequences_over_the_chain_forms`, restated in this node's terms as: the kind the entitlement form reports is the kind whose ABI type name stands at the entitlement's positions in the field sequences.
- `chain_forms_interval_survives_its_field_round_trip` (`chain/create_chain_forms`): absorbed for its `KIND` outcome; proven by the `integration_test.rs` block for `the_entries_carry_the_mint_and_transfer_field_sequences_over_the_chain_forms`, restated in this node's terms as: the kind the interval form reports is the kind whose ABI type name stands at the interval's positions in the field sequences.
- `chain_forms_chain_identifier_survives_its_field_round_trip` (`chain/create_chain_forms`): absorbed for its `KIND` outcome; proven by the `integration_test.rs` block for `the_entries_carry_the_mint_and_transfer_field_sequences_over_the_chain_forms`, restated in this node's terms as: the kind the chain identifier form reports is the kind whose ABI type name stands at the chain identifier's positions in the field sequences.
- `libraries_sharing_an_identifier_return_the_same_scalar_field_order` (`pairing/factory`): absorbed; proven by the `integration_test.rs` block for `libraries_sharing_an_identifier_carry_the_same_scalar_field_order`, restated in this node's terms as: the `SCALAR_FIELD_ORDER` entry holds the order's 32 big-endian bytes as a `SolidityConstantValue::Uint256`, equal across any two concretes whose declared target-group encoding identifiers are equal.

## Integration: own entries

Each entry is proven by the private integration block of the same name. Every entry runs `constants` at `DELIVERY_STATEMENT_VERSION_ONE` over every member of `PAIRING_CONCRETES` and every member of `CHAIN_FORMS_CONCRETES`, with the payload built from the official declaration builders, and reads each entry from the returned entries by its constant name.

- `the_entries_carry_each_declarations_domain_tag_under_its_own_name`: condition the KEM, key-agreement, and delivery-proof declarations are built carrying the tags each family's real concrete declares, as `kem/bb1_depth_one`, `envelope/pairing_elgamal`, and `proof/schnorr_fs` state them; outcome `Ok` whose entries `IDENTITY_TAG`, `POSSESSION_G1_TAG`, `POSSESSION_G2_TAG`, `CHALLENGE_TAG`, and `WEIGHT_TAG` are `Bytes` holding the KEM declaration's `identity_tag`, the key agreement's `possession_g1_tag` and `possession_g2_tag`, and the delivery proof's `challenge_tag` and `weight_tag`, each equal to the literal its owning node states for that tag; variation every concrete of both declared sets, the tags differing from each other and from every builder default, so that an entry read from another field, from a builder default, or under another name fails; edge each tag is read from the declaration the payload carries and `constants` holds none.
- `the_entries_carry_the_statement_version_and_each_purpose_code_in_declared_order`: condition the params carry `DELIVERY_STATEMENT_VERSION_ONE`; outcome `Ok` whose entry `DELIVERY_STATEMENT_VERSION` is `Uint16` holding the value of `DELIVERY_STATEMENT_VERSION_ONE` and whose entries `PURPOSE_MINT`, `PURPOSE_TRANSFER`, `PURPOSE_GRANT`, and `PURPOSE_REPLACEMENT` are `Uint16` holding the codes `DeliveryPurpose::Mint`, `Transfer`, `Grant`, and `Replacement` declare, each the literal `proof/mint_statement` states, the purposes' entries following `DELIVERY_PURPOSES` order; variation every concrete of both declared sets, the codes differing from each other so that swapped codes or a reordering fail; edge the codes are the purposes' own and the entry names are the function's.
- `the_entries_carry_the_mint_and_transfer_field_sequences_over_the_chain_forms`: condition the forms the chain family hands the consumer are the concrete's; outcome `Ok` whose entries `MINT_FIELDS` and `TRANSFER_FIELDS` are `String` holding the ABI type names of `MintStatementDescription::FIELDS` and `TransferStatementDescription::FIELDS` joined by `,` in field order, each equal to the literal the block assembles from the field kinds the description's interaction spec lists, the forms' kinds in the `context_slice`, and the table `render` states; variation every concrete of both declared sets, each sequence compared with its own literal; edge a fixed twenty-byte kind is `bytes20`, never `address`.
- `the_entries_carry_the_scalar_field_order_as_32_big_endian_bytes`: condition the pairing handed is a concrete of `PAIRING_CONCRETES`; outcome `Ok` whose entry `SCALAR_FIELD_ORDER` is `Uint256` whose 32 bytes the same pairing's `decode_scalar` refuses with `NonCanonical` and whose bytes less one it admits; variation every concrete of `PAIRING_CONCRETES`, so a constant, a truncated, a little-endian, or an off-by-one order fails; edge the order is read through the family's reference trait and checked through its arithmetic trait.
- `the_entries_carry_the_generators_precompile_encodings`: condition the pairing handed is a concrete of `PAIRING_CONCRETES`; outcome `Ok` whose entries `G1_GENERATOR` and `G2_GENERATOR` are `Bytes` that the same pairing's `decode_g1` and `decode_g2` admit, each decoded point's sum with the negation of that group's generator being the identity; variation every concrete of `PAIRING_CONCRETES`, so an entry holding the other group's generator or another point fails at the decoder; edge the bytes are the encoding the pairing's decoders read.
- `the_entries_carry_the_verifier_form_the_pairing_declares`: condition the pairing handed is a concrete of `PAIRING_CONCRETES`; outcome `Ok` whose entry `SECOND_GROUP_ARITHMETIC` is `Bool(false)` for a pairing whose declaration carries `PrecompileEncoding::Eip196Eip197`, which has no second-group precompile, and `Bool(true)` for `PrecompileEncoding::Eip2537`, which has both groups, the expectation keyed on the precompile encoding and never on the `verifier_group_arithmetic` field the function reads; variation every concrete of `PAIRING_CONCRETES`, which holds a pairing of each encoding, so a constant fails; edge the form is read from the declaration of the pairing handed.
- `the_entries_are_listed_in_declared_order`: condition `constants` is called over a concrete of each declared set; outcome `Ok` whose entry names, in order, are `IDENTITY_TAG`, `POSSESSION_G1_TAG`, `POSSESSION_G2_TAG`, `CHALLENGE_TAG`, `WEIGHT_TAG`, `DELIVERY_STATEMENT_VERSION`, `PURPOSE_MINT`, `PURPOSE_TRANSFER`, `PURPOSE_GRANT`, `PURPOSE_REPLACEMENT`, `MINT_FIELDS`, `TRANSFER_FIELDS`, `SCALAR_FIELD_ORDER`, `G1_GENERATOR`, `G2_GENERATOR`, and `SECOND_GROUP_ARITHMETIC`; variation every concrete of both declared sets; edge the order is the function's and not the declarations'.

## Integration: private surface

The chain of real functions is `create_pairing`, the arm's concrete through its `try_new`, `consume_pairing`, `create_chain_forms`, the concrete's producer, `consume_chain_forms`, `constants`, `SolidityConstantName::try_new`, `SolidityStringLiteral::try_new`, `SolidityUint256::try_new`, `SolidityAbiType::try_new` and `as_str`, and the pairing methods the blocks call as the oracle; no outer-edge collaborator is mocked, the declarations being builder data and the function reading no clock, drawing no randomness, and touching no filesystem; the observable result is the `ConstantsReturn` and the pairing's decoder and group-operation results; the entries proven are every own entry above and the absorbed entries.

