# `abi` — interaction spec

Branch contract for the `abi` module of the `encoding` crate: `AbiEncoding`, the encoding family's first concrete, the adapter over `alloy`'s dynamic ABI implementing `IEncoderAdapter` and `IDecoderAdapter`. Each branch states condition, decision, dependency call, and the exact return outcome.

## `IEncoderAdapter::type_name(params: TypeNameParams, payload: CanonicalFieldKind) -> TypeNameReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| kind named | any `CanonicalFieldKind` | a `match` on the kind, exhaustive, so a kind with no arm fails to compile | none | `Ok(TypeNameSuccessReturn { name })`, `name` the `EncodingTypeName` built from the table `FixedBytes32` `bytes32`, `Unsigned16` `uint16`, `Unsigned32` `uint32`, `Unsigned64` `uint64`, `Text` `string`, `Bytes` `bytes`, `FixedBytes20` `bytes20` — never `address`, and `Unsigned256` `uint256`, each built by `EncodingTypeName::try_new` over the name the table states, its success destructured irrefutably; the error arm has no branch |

## `EncodingTypeName::try_new(params: EncodingTypeNameConstructorParams) -> EncodingTypeNameTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| empty name | `params.name` has no characters | the empty check, first | none | `Err(EncodingTypeNameErrorReturn::Empty)` |
| non-alphanumeric name | `params.name` holds a character other than an ASCII letter or digit | the character check, decided after the empty check | none | `Err(EncodingTypeNameErrorReturn::NotAlphanumeric)` |
| admitted name | `params.name` is non-empty and holds only ASCII letters and digits | both checks pass | none | `Ok(EncodingTypeName)` holding the text |

## `AbiEncoding::try_new(params: AbiEncodingConstructorParams) -> AbiEncodingTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| admitted | any params | none | none | `Ok(AbiEncoding)`; the error arm has no branch |

## `AbiEncoding::DECLARATION`

The inherent constant `EncodingDeclaration { identifier: EncodingIdentifier::EthereumAbiV1, adapter_version: 1, interface_version: ENCODING_INTERFACE_VERSION }`.

## `IEncoderAdapter::encode<D: IEncodingContract>(&self, params: EncodeParams<'_, D>, payload: &D::Described) -> EncodeReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| wrong field count | the count `params.description.to_fields` emits differs from `D::FIELDS.len()` | the count comparison, before ABI encoding | `params.description.to_fields(ToFieldsParams, payload)` once, its infallible result unpacked | `Err(EncodeErrorReturn::FieldCount { expected, actual })` |
| wrong field kind | the count passes and an emitted value's variant differs from the `D::FIELDS` kind at its index | the per-index kind comparison, before ABI encoding | none further | `Err(EncodeErrorReturn::FieldKind { index, expected, actual })` for the lowest mismatching index |
| admitted fields | every emitted value matches `D::FIELDS` in count and kind | all checks pass | `DynSolValue::abi_encode_params` once over the `DynSolValue::Tuple` collecting, in order, `FixedBytes32(bytes)` to `DynSolValue::FixedBytes(B256::from(bytes), 32)`, `Unsigned16(value)`, `Unsigned32(value)`, `Unsigned64(value)` to `DynSolValue::Uint(U256::from(value), 16)`, `32`, `64`, `Text(text)` to `DynSolValue::String(text)`, `Bytes(bytes)` to `DynSolValue::Bytes(bytes)`, `FixedBytes20(bytes)` to `DynSolValue::FixedBytes(B256::right_padding_from(&bytes), 20)`, `Unsigned256(bytes)` to `DynSolValue::Uint(U256::from_be_bytes(bytes), 256)` | `Ok(EncodeSuccessReturn { bytes })`, the parameter encoding |

## `IDecoderAdapter::decode<D: IEncodingContract>(&self, params: DecodeParams<'_, D>, payload: &[u8]) -> DecodeReturn<D::Described, D::FromFieldsErrorReturn>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| malformed | `DynSolType::Tuple` over the description's `FIELDS`, each kind mapped to `DynSolType::FixedBytes(32)`, `Uint(16)`, `Uint(32)`, `Uint(64)`, `String`, `Bytes`, `FixedBytes(20)`, or `Uint(256)`, refuses the payload | the decoder's result | `abi_decode_params` once | `Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::Malformed(error)))`, the error unchanged |
| shape mismatch | the decoded value is not a tuple, or holds at some index of `FIELDS` no item or an item not of the kind named there | the items read in ascending index order: a `bytes32` item `as_fixed_bytes` of size `32` converting into `[u8; 32]`; a `bytes20` item `as_fixed_bytes` of size `20`, its word's first twenty bytes converting into `[u8; 20]`; an integer item `as_uint` of the field's width; a `uint256` item's word taken as its big-endian `[u8; 32]`; a text item `as_str`; a byte-string item `as_bytes` copied into a `Vec<u8>` as `CanonicalFieldValue::Bytes` | none | `Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::DecodedShapeMismatch { index }))` for the lowest such index, `0` when the value is not a tuple; the decoder returns the type it was given, so no input takes this branch and it has no unit test |
| integer out of range | an integer item's value does not convert into the field's `u16`, `u32`, or `u64` through `try_from` | the conversion, in the same ascending pass | none | `Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::ValueOutOfRange { index, kind }))` for the lowest such index, `kind` the field's kind; a 256-bit field never takes this branch, since every word fits it |
| description refused | every item converts | the description's result | `params.description.fields_to_value(FromFieldsParams, CanonicalFields { values })` once over the converted values in field order | `Err(DecodeErrorReturn::Description(error))`, the refusal unchanged |
| encoder contract refused | the description admits the value and re-encoding returns `Err(error)` | the re-encoder's result | `self.encode(EncodeParams { description: params.description }, &described)` once | `Err(DecodeErrorReturn::EncoderContract(error))` |
| non-canonical | the description admits the value and re-encoding succeeds with bytes other than the payload | byte equality | `self.encode` once | `Err(DecodeErrorReturn::NonCanonical)`; this refuses trailing bytes, nonzero padding, non-canonical offsets, and a head of the wrong length, which the dynamic decoder reads past |
| admitted | the re-encoding equals the payload | byte equality | none further | `Ok(DecodeSuccessReturn { described })` |

## Ordering

The ABI decode precedes the item pass, the item pass precedes the description, and the re-encoding comparison runs last; `params` supplies the description and nothing else is read from it.

## Integration: own entries and callee dispositions

No callee entry and no own entry: the concrete's callees are `alloy`, a vendor library, and the contract methods of the description its caller hands it.
