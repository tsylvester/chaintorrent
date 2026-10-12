# abi interactions

`AbiEncoding` is the encoding family's first concrete: the adapter over `alloy`'s dynamic ABI. Its fallible constructor `try_new` is its only producer; `AbiEncoding::DECLARATION` names the encoding identifier it sits under, its adapter version, and the interface version it implements, readable before any instance exists. It implements `IEncoderAdapter` and `IDecoderAdapter`.

## `AbiEncoding::try_new`

`AbiEncoding::try_new(params: AbiEncodingConstructorParams) -> AbiEncodingTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| admitted | any params | none | none | `Ok(AbiEncoding)`; the error arm has no branch |

## `AbiEncoding::DECLARATION`

The inherent constant `EncodingDeclaration { identifier: EncodingIdentifier::EthereumAbiV1, adapter_version: 1, interface_version: ENCODING_INTERFACE_VERSION }`.

## `IEncoderAdapter::encode`

`AbiEncoding::encode<D: IEncodingContract>(params: EncodeParams<'_, D>, payload: &D::Described) -> EncodeReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| wrong field count | the emitted count differs from `D::FIELDS.len()` | the count comparison, before ABI encoding | `params.description.to_fields(ToFieldsParams, payload)` once, its infallible result unpacked | `Err(EncodeErrorReturn::FieldCount { expected, actual })` |
| wrong field kind | the count passes and an emitted value's variant differs from the corresponding `D::FIELDS` kind | the kind comparison in index order, before ABI encoding | none beyond the same `to_fields` call | `Err(EncodeErrorReturn::FieldKind { index, expected, actual })` for the lowest mismatch |
| admitted fields | every emitted value matches `D::FIELDS` in count and kind | none | `DynSolValue::abi_encode_params` once | `Ok(EncodeSuccessReturn { bytes })`, `bytes` the parameter encoding of `DynSolValue::Tuple` holding each canonical value mapped in order — `FixedBytes32(bytes)` to `DynSolValue::FixedBytes(B256::from(bytes), 32)`, `Unsigned16`, `Unsigned32`, and `Unsigned64` to `DynSolValue::Uint(U256::from(value), 16)`, `32`, and `64`, `Text(text)` to `DynSolValue::String(text)`, `Bytes(bytes)` to `DynSolValue::Bytes(bytes)`, `FixedBytes20(bytes)` to `DynSolValue::FixedBytes(B256::right_padding_from(&bytes), 20)`, and `Unsigned256(bytes)` to `DynSolValue::Uint(U256::from_be_bytes(bytes), 256)` |

`params` supplies the description and nothing else is read from it.

## `IDecoderAdapter::decode`

`AbiEncoding::decode<D: IEncodingContract>(params: DecodeParams<'_, D>, payload: &[u8]) -> DecodeReturn<D::Described, D::FromFieldsErrorReturn>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| malformed | the payload is not the ABI parameter encoding of `DynSolType::Tuple` over the description's `FIELDS`, each kind mapped to `DynSolType::FixedBytes(32)`, `DynSolType::Uint(16)`, `DynSolType::Uint(32)`, `DynSolType::Uint(64)`, `DynSolType::String`, `DynSolType::Bytes`, `DynSolType::FixedBytes(20)`, or `DynSolType::Uint(256)` | the decoder's result | `DynSolType::abi_decode_params` once | `Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::Malformed(error)))`, the error unchanged |
| shape mismatch | the decoded value is not a tuple, or holds at some index of `FIELDS` no item or an item not of the kind named there — a `bytes32` item being `as_fixed_bytes` of size `32` converting into `[u8; 32]`, a `bytes20` item `as_fixed_bytes` of size `20` whose word's first twenty bytes convert into `[u8; 20]`, an integer item `as_uint` of the field's width, a `uint256` item's word taken as its big-endian `[u8; 32]`, a text item `as_str`, and a byte-string item `as_bytes`, copied into a `Vec<u8>` as `CanonicalFieldValue::Bytes` | the fields are read in ascending index order | none | `Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::DecodedShapeMismatch { index }))` for the lowest such index, `0` when the value is not a tuple; the decoder returns the type it was given, so no input takes this branch and it has no unit test |
| integer out of range | an integer item's value does not convert into the field's `u16`, `u32`, or `u64` through `try_from` | the conversion, in the same ascending pass | none | `Err(DecodeErrorReturn::Abi(AbiDecoderErrorReturn::ValueOutOfRange { index, kind }))` for the lowest such index, `kind` the field's kind; a 256-bit field never takes this branch, since every word fits it |
| description refused | every item converts | the description's result | `params.description.fields_to_value(FromFieldsParams, CanonicalFields { values })` once over the converted values in field order | `Err(DecodeErrorReturn::Description(error))`, the refusal unchanged |
| encoder contract refused | the description admits the value and re-encoding returns `Err(error)` | the encode call's result | `self.encode(EncodeParams { description: params.description }, &described)` once | `Err(DecodeErrorReturn::EncoderContract(error))`, the schema refusal unchanged |
| non-canonical | the description admits the value and the re-encoding succeeds with bytes other than the payload | byte equality | `encode` once, the same call | `Err(DecodeErrorReturn::NonCanonical)`; this refuses trailing bytes, nonzero padding, non-canonical offsets, and a head of the wrong length, which the dynamic decoder reads past |
| admitted | the re-encoding equals the payload | the same | the same call | `Ok(DecodeSuccessReturn { described })` |

Ordering: the ABI decode precedes the item pass, the item pass precedes the description, and the re-encoding comparison runs last; `params` supplies the description and nothing else is read from it.

## Invariants

- For any description and admitted value, `decode` over `encode`'s output returns that value: exactly one byte form per value.

## Integration: callee dispositions

The `encoding/derivation_context` entries this node's route reaches, each dispositioned as stated.

- `a_context_survives_the_round_trip_through_its_canonical_fields`: absorbed; proven by `a_context_survives_the_abi_round_trip` in the private integration test element `abi/integration_test.rs`, whose context carries the entry's variation and whose fields pass through the ABI byte form between `to_fields` and `fields_to_value`.

## Integration: own entries

Each entry is named and proven by the private integration block of the same name.

- `reference_context_encodes_to_the_reference_vector`: condition the reference derivation context is handed to `encode` with `DerivationContextDescription`; outcome `Ok(EncodeSuccessReturn { bytes })` with `bytes` the known-answer vector authored from the ABI specification for the reference fields; variation a context whose text fields are dynamic items between static items and whose same-kind fields differ, so a swapped field, a wrong word width, or a misplaced tail changes the bytes; edge that must survive the composition: the description's emitted order and kinds are the order and kinds `FIELDS` declares, so the encoder's check admits them.
- `reference_vector_decodes_to_the_reference_context`: condition the reference vector is handed to `decode` with `DerivationContextDescription`; outcome `Ok(DecodeSuccessReturn { described })` with `described` equal to the reference context; variation the same context, so a field read from the wrong word or tail yields a refusal or an unequal context; edge that must survive the composition: the canonical re-encoding of the described context equals the vector, so the one admitted byte form is the byte form `encode` writes.
- `a_context_survives_the_abi_round_trip`: condition an admitted `DerivationContext` is handed to `encode` and the bytes it returns are handed to `decode`; outcome `Ok(DecodeSuccessReturn { described })` with `described` equal to the original context; variation a context whose same-kind fields hold distinct values, whose 16-bit suite version is at its maximum, whose piece-group size exceeds the `i32` range, and whose group index and total extent exceed the 32-bit range, so a swap of same-kind fields, a dropped field, or a narrowed width yields a refusal or an unequal context; edge that must survive the composition: the rebuilt context is admitted at the full width of the extent, and the canonical comparison accepts every byte string `encode` writes.
- `a_value_the_description_refuses_is_refused_with_the_description_refusal`: condition a canonical ABI encoding whose deployment-identity word is all zero is handed to `decode` with `DerivationContextDescription`; outcome `Err(DecodeErrorReturn::Description(DerivationContextFromFieldsErrorReturn::DeploymentIdentity(DeploymentIdentityTryNewErrorReturn::AllZero)))`; variation an encoding that is canonical and well-kinded, so the refusal comes from the domain's constructor through the description and not from the ABI decode or the canonical comparison; edge that must survive the composition: the refusal passes through the decoder unchanged and whole.

## Integration: private surface

The chain of real functions `AbiEncoding::encode` and `AbiEncoding::decode`, `DerivationContextDescription::to_fields` and `fields_to_value`, and `domain`'s accessors and constructors; no outer-edge collaborator is mocked, `alloy` running as the concrete's own vendor library; the observable result is the bytes `encode` returns and the context or the refusal `decode` returns.

