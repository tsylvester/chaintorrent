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

