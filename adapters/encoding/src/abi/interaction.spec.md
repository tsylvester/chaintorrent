# abi interactions

`AbiEncoding` is the encoding family's first concrete: the adapter over
`alloy`'s dynamic ABI. Its fallible constructor `try_new` is its only
producer; `AbiEncoding::DECLARATION` names the encoding identifier it sits
under, its adapter version, and the interface version it implements, readable
before any instance exists. It implements `IEncoderAdapter` and
`IDecoderAdapter`.

## `AbiEncoding::try_new`

`AbiEncoding::try_new(params: AbiEncodingConstructorParams) -> AbiEncodingTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| admitted | any params | none | none | `Ok(AbiEncoding)`; the error arm has no branch |

## `AbiEncoding::DECLARATION`

The inherent constant
`EncodingDeclaration { identifier: EncodingIdentifier::EthereumAbiV1, adapter_version: 1, interface_version: ENCODING_INTERFACE_VERSION }`.

## `IEncoderAdapter::encode`

`AbiEncoding::encode<D: IEncodingContract>(params: EncodeParams<'_, D>, payload: &D::Described) -> EncodeReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| admitted | any description and admitted value | none | `params.description.to_fields(ToFieldsParams, payload)` once, unpacked irrefutably, then `DynSolValue::abi_encode_params` once | `Ok(EncodeSuccessReturn { bytes })`, `bytes` the parameter encoding of `DynSolValue::Tuple` holding each canonical value mapped in order — `FixedBytes32(bytes)` to `DynSolValue::FixedBytes(B256::from(bytes), 32)`, `Unsigned16`, `Unsigned32`, and `Unsigned64` to `DynSolValue::Uint(U256::from(value), 16)`, `32`, and `64`, and `Text(text)` to `DynSolValue::String(text)`, `Bytes(bytes)` to `DynSolValue::Bytes(bytes)`, `FixedBytes20(bytes)` to `DynSolValue::FixedBytes(B256::right_padding_from(&bytes), 20)`, and `Unsigned256(bytes)` to `DynSolValue::Uint(U256::from_be_bytes(bytes), 256)` |

`params` supplies the description and nothing else is read from it.

## `IDecoderAdapter::decode`

`AbiEncoding::decode<D: IEncodingContract>(params: DecodeParams<'_, D>, payload: &[u8]) -> DecodeReturn<D::Described, D::FromFieldsErrorReturn>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| malformed | the payload is not the ABI parameter encoding of `DynSolType::Tuple` over the description's `FIELDS`, each kind mapped to `DynSolType::FixedBytes(32)`, `DynSolType::Uint(16)`, `DynSolType::Uint(32)`, `DynSolType::Uint(64)`, `DynSolType::String`, `DynSolType::Bytes`, `DynSolType::FixedBytes(20)`, or `DynSolType::Uint(256)` | the decoder's result | `DynSolType::abi_decode_params` once | `Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::Malformed(error)))`, the error unchanged |
| shape mismatch | the decoded value is not a tuple, or holds at some index of `FIELDS` no item or an item not of the kind named there — a `bytes32` item being `as_fixed_bytes` of size `32` converting into `[u8; 32]`, a `bytes20` item `as_fixed_bytes` of size `20` whose word's first twenty bytes convert into `[u8; 20]`, an integer item `as_uint` of the field's width, a `uint256` item's word taken as its big-endian `[u8; 32]`, a text item `as_str`, and a byte-string item `as_bytes`, copied into a `Vec<u8>` as `CanonicalFieldValue::Bytes` | the fields are read in ascending index order | none | `Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::DecodedShapeMismatch { index }))` for the lowest such index, `0` when the value is not a tuple; the decoder returns the type it was given, so no input takes this branch and it has no unit test |
| integer out of range | an integer item's value does not convert into the field's `u16`, `u32`, or `u64` through `try_from` | the conversion, in the same ascending pass | none | `Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::ValueOutOfRange { index, kind }))` for the lowest such index, `kind` the field's kind; a 256-bit field never takes this branch, since every word fits it |
| description refused | every item converts | the description's result | `params.description.fields_to_value(FromFieldsParams, CanonicalFields { values })` once over the converted values in field order | `Err(DecodeErrorReturn::Description(error))`, the refusal unchanged |
| non-canonical | the description admits the value and `self.encode(EncodeParams { description: params.description }, &described)` returns bytes other than the payload | byte equality | `encode` once | `Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::NonCanonical))`; this refuses trailing bytes, nonzero padding, non-canonical offsets, and a head of the wrong length, which the dynamic decoder reads past |
| admitted | the re-encoding equals the payload | the same | the same call | `Ok(DecodeSuccessReturn { described })` |

Ordering: the ABI decode precedes the item pass, the item pass precedes the
description, and the re-encoding comparison runs last; `params` supplies the
description and nothing else is read from it.

## Invariants

- For any description and admitted value, `decode` over `encode`'s output
  returns that value: exactly one byte form per value.
