# `evm_forms` — interaction spec

Branch contract for the `evm_forms` module of the `chain` crate: the EVM suite's forms as the chain family's first concrete — the identity form over twenty bytes, the entitlement and chain-identifier forms over 256-bit unsigned words, the interval form over a 64-bit unsigned value, and the key-identifier and signed-intent forms over fixed 32 bytes, each implementing `ICanonicalField`, the identity form deriving a key identifier from a public key's canonical bytes and a scheme identifier, and `EvmForms` implementing `IChainForms` over them and producing the signed intent as an EIP-712 digest. Each branch states condition, decision, dependency call, and the exact return outcome.

## `EvmForms::try_new<E: IEncoderAdapter>(params: EvmFormsConstructorParams<'_, E>) -> EvmFormsTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| construct | any params | none | `params.encoder.type_name(TypeNameParams, kind)` once for each of the eight canonical field kinds, each success destructured irrefutably because its error arm is uninhabited | `Ok(EvmForms { type_names })` with `type_names` holding the name the encoder returned for each kind in the field of that kind |

The error arm has no branch: every canonical field kind has a name, so `Infallible` is uninhabited. The constructor names no encoding concrete and holds no name of its own.

## `EvmForms::DECLARATION`

An inherent constant, readable from the type before any instance exists:

`ChainFormsDeclaration { identifier: ChainFormsIdentifier::EvmV1, adapter_version: 1, interface_version: CHAIN_FORMS_INTERFACE_VERSION, encoding_identifier: EncodingIdentifier::EthereumAbiV1, key_identifier_domain_tag: b"chaintorrent/evm/key-identifier/v1", intent_domain_name: "ChainTorrent", intent_domain_version: "1", intent_struct_name: "Intent", intent_member_names: IntentMemberNames { identity: "identity", nonce: "nonce", expiry: "expiry", call: "call" }, nonce_kind: CanonicalFieldKind::Unsigned64, expiry_kind: CanonicalFieldKind::Unsigned64 }`

## `IChainForms` for `EvmForms`

`const DECLARATION: ChainFormsDeclaration = EvmForms::DECLARATION;` `Identity` is `EvmIdentityForm`, `Entitlement` is `EvmEntitlementForm`, `Interval` is `EvmIntervalForm`, `ChainIdentifier` is `EvmChainIdentifierForm`, and `SignedIntent` is `EvmSignedIntentForm`.

## `IIdentityForm` for `EvmIdentityForm`

`KeyIdentifier` is `EvmKeyIdentifierForm`.

## `EvmIdentityForm::derive_key_identifier(params: DeriveKeyIdentifierParams, payload: DeriveKeyIdentifierPayload<'_>) -> DeriveKeyIdentifierReturn<EvmKeyIdentifierForm>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| derive | any payload | none | a `Keccak256` hasher updated with `EvmForms::DECLARATION.key_identifier_domain_tag`, then the one byte `payload.scheme.identifier()`, then `payload.key`, in that order, finalized; then `EvmKeyIdentifierForm::try_new(EvmKeyIdentifierFormConstructorParams { bytes })` over the 32-byte digest, once, unpacked by `let Ok(key_identifier) = …;` | `Ok(DeriveKeyIdentifierSuccessReturn { key_identifier })` |

The error arm has no branch: every key and scheme has an identifier. `params` carries no control and is not read. The rule is the same for every scheme.

## `EvmForms::signed_intent(&self, params: SignedIntentParams, payload: SignedIntentPayload<'_, EvmForms>) -> SignedIntentReturn<EvmSignedIntentForm>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| operation not an identifier | `payload.call.operation` is empty, begins with other than an ASCII letter or underscore, or holds a character other than an ASCII letter, digit, or underscore | the character check | none | `Err(SignedIntentErrorReturn::OperationNotIdentifier { operation })` with the operation as given |
| operation named as the struct | the operation equals `EvmForms::DECLARATION.intent_struct_name`, decided after the identifier check | the comparison | none | `Err(SignedIntentErrorReturn::OperationIsStructName { operation })` |
| field not an identifier | the field at the lowest index whose name fails the identifier check, decided after the operation checks | the per-field character check | none | `Err(SignedIntentErrorReturn::FieldNotIdentifier { name })` |
| field repeated | the field at the lowest index whose name equals an earlier field's name, each field's identifier check preceding its repetition check | the per-field comparison against the earlier names | none | `Err(SignedIntentErrorReturn::FieldRepeated { name })` |
| nonce of another kind | the kind of `payload.nonce` is not `EvmForms::DECLARATION.nonce_kind`, decided after the field checks | the kind comparison | none | `Err(SignedIntentErrorReturn::NonceKind { expected, actual })` with the declared and the given kind |
| expiry of another kind | the kind of `payload.expiry` is not `EvmForms::DECLARATION.expiry_kind`, decided after the nonce check | the kind comparison | none | `Err(SignedIntentErrorReturn::ExpiryKind { expected, actual })` with the declared and the given kind |
| admitted | every check above passes | all checks pass | `to_field` once on each of `payload.chain`, `payload.contract`, and `payload.identity`, each success destructured irrefutably, and `Keccak256` over the EIP-712 pieces below, then `EvmSignedIntentForm::try_new(EvmSignedIntentFormConstructorParams { bytes })` over the 32-byte digest, once, unpacked by `let Ok(signed_intent) = …;` | `Ok(SignedIntentSuccessReturn { signed_intent })` |

The EIP-712 pieces are built in this order. The intent struct's type string is the declared struct name, an opening parenthesis, the `FixedBytes20` type name and the declared identity member name, the nonce kind's type name and the declared nonce member name, the expiry kind's type name and the declared expiry member name, the operation and the declared call member name, each type then a space then its member, joined by commas, and a closing parenthesis; then the call's type string is the operation, an opening parenthesis, each field's kind's type name then a space then its name, joined by commas in field order, and a closing parenthesis. The intent's type digest is the digest of the intent struct's type string followed directly by the call's type string, and the call's type digest is the digest of the call's type string alone. Every type name is the one `self.type_names` holds for the kind, so no name is written in this function.

The call's hash is the digest of the call's type digest followed by one 32-byte word per field value. The intent's hash is the digest of the intent's type digest, the identity's word, the nonce's word, the expiry's word, and the call's hash. A word is the EIP-712 encoding of a canonical field value: `FixedBytes32` and `Unsigned256` as their 32 bytes, `Unsigned16`, `Unsigned32`, and `Unsigned64` as their big-endian bytes left-padded with zeros to 32, `FixedBytes20` as its 20 bytes right-padded with zeros to 32, and `Text` and `Bytes` as the digest of their bytes, a text as its UTF-8 bytes.

The domain separator is the digest of the digest of `EIP712_DOMAIN_TYPE`, the digest of the declared domain name, the digest of the declared domain version, the chain's 256-bit field, and the contract's 20 bytes left-padded with zeros to 32. The output is the digest of `EIP712_PREFIX`, the domain separator, and the intent's hash.

The output changes with each of the chain, the contract, the acting identity, the nonce, the expiry, the operation, and every call field's name and value; the same payload always yields the same output; no call has a side effect; `params` carries no control and is not read.

## `EvmIdentityForm::try_new(params: EvmIdentityFormConstructorParams) -> EvmIdentityFormTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| construct | any params | none | none | `Ok(EvmIdentityForm { bytes })`, the array moved from the params |

The error arm has no branch: `Infallible` is uninhabited.

## `ICanonicalField` for `EvmIdentityForm`

`KIND` is `CanonicalFieldKind::FixedBytes20`.

## `EvmIdentityForm::to_field(params: ToFieldParams, payload: &Self) -> ToFieldReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| yielded | any form | none | none | `Ok(ToFieldSuccessReturn { field: CanonicalFieldValue::FixedBytes20(payload.bytes) })`, the array copied |

`params` carries no control and is not read.

## `EvmIdentityForm::from_field(params: FromFieldParams, payload: CanonicalFieldValue) -> FromFieldReturn<Self>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| wrong kind | `payload` is not `CanonicalFieldValue::FixedBytes20` | `let CanonicalFieldValue::FixedBytes20(bytes) = payload else { … }` | none | `Err(FromFieldErrorReturn::WrongKind { expected: CanonicalFieldKind::FixedBytes20 })` |
| admitted | `payload` is `CanonicalFieldValue::FixedBytes20(bytes)` | the same match | `EvmIdentityForm::try_new(EvmIdentityFormConstructorParams { bytes })`, once, unpacked by `let Ok(value) = …;` | `Ok(FromFieldSuccessReturn { value })` |

## `EvmEntitlementForm::try_new(params: EvmEntitlementFormConstructorParams) -> EvmEntitlementFormTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| construct | any params | none | none | `Ok(EvmEntitlementForm { bytes })`, the array moved from the params |

The error arm has no branch: `Infallible` is uninhabited.

## `ICanonicalField` for `EvmEntitlementForm`

`KIND` is `CanonicalFieldKind::Unsigned256`.

## `EvmEntitlementForm::to_field(params: ToFieldParams, payload: &Self) -> ToFieldReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| yielded | any form | none | none | `Ok(ToFieldSuccessReturn { field: CanonicalFieldValue::Unsigned256(payload.bytes) })`, the array copied |

`params` carries no control and is not read.

## `EvmEntitlementForm::from_field(params: FromFieldParams, payload: CanonicalFieldValue) -> FromFieldReturn<Self>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| wrong kind | `payload` is not `CanonicalFieldValue::Unsigned256` | `let CanonicalFieldValue::Unsigned256(bytes) = payload else { … }` | none | `Err(FromFieldErrorReturn::WrongKind { expected: CanonicalFieldKind::Unsigned256 })` |
| admitted | `payload` is `CanonicalFieldValue::Unsigned256(bytes)` | the same match | `EvmEntitlementForm::try_new(EvmEntitlementFormConstructorParams { bytes })`, once, unpacked by `let Ok(value) = …;` | `Ok(FromFieldSuccessReturn { value })` |

## `EvmIntervalForm::try_new(params: EvmIntervalFormConstructorParams) -> EvmIntervalFormTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| construct | any params | none | none | `Ok(EvmIntervalForm { value })`, the value moved from the params |

The error arm has no branch: `Infallible` is uninhabited.

## `ICanonicalField` for `EvmIntervalForm`

`KIND` is `CanonicalFieldKind::Unsigned64`.

## `EvmIntervalForm::to_field(params: ToFieldParams, payload: &Self) -> ToFieldReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| yielded | any form | none | none | `Ok(ToFieldSuccessReturn { field: CanonicalFieldValue::Unsigned64(payload.value) })` |

`params` carries no control and is not read.

## `EvmIntervalForm::from_field(params: FromFieldParams, payload: CanonicalFieldValue) -> FromFieldReturn<Self>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| wrong kind | `payload` is not `CanonicalFieldValue::Unsigned64` | `let CanonicalFieldValue::Unsigned64(value) = payload else { … }` | none | `Err(FromFieldErrorReturn::WrongKind { expected: CanonicalFieldKind::Unsigned64 })` |
| admitted | `payload` is `CanonicalFieldValue::Unsigned64(value)` | the same match | `EvmIntervalForm::try_new(EvmIntervalFormConstructorParams { value })`, once, unpacked by `let Ok(value) = …;` | `Ok(FromFieldSuccessReturn { value })` |

## `EvmChainIdentifierForm::try_new(params: EvmChainIdentifierFormConstructorParams) -> EvmChainIdentifierFormTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| construct | any params | none | none | `Ok(EvmChainIdentifierForm { bytes })`, the array moved from the params |

The error arm has no branch: `Infallible` is uninhabited.

## `ICanonicalField` for `EvmChainIdentifierForm`

`KIND` is `CanonicalFieldKind::Unsigned256`.

## `EvmChainIdentifierForm::to_field(params: ToFieldParams, payload: &Self) -> ToFieldReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| yielded | any form | none | none | `Ok(ToFieldSuccessReturn { field: CanonicalFieldValue::Unsigned256(payload.bytes) })`, the array copied |

`params` carries no control and is not read.

## `EvmChainIdentifierForm::from_field(params: FromFieldParams, payload: CanonicalFieldValue) -> FromFieldReturn<Self>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| wrong kind | `payload` is not `CanonicalFieldValue::Unsigned256` | `let CanonicalFieldValue::Unsigned256(bytes) = payload else { … }` | none | `Err(FromFieldErrorReturn::WrongKind { expected: CanonicalFieldKind::Unsigned256 })` |
| admitted | `payload` is `CanonicalFieldValue::Unsigned256(bytes)` | the same match | `EvmChainIdentifierForm::try_new(EvmChainIdentifierFormConstructorParams { bytes })`, once, unpacked by `let Ok(value) = …;` | `Ok(FromFieldSuccessReturn { value })` |

## `EvmKeyIdentifierForm::try_new(params: EvmKeyIdentifierFormConstructorParams) -> EvmKeyIdentifierFormTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| construct | any params | none | none | `Ok(EvmKeyIdentifierForm { bytes })`, the array moved from the params |

The error arm has no branch: `Infallible` is uninhabited.

## `ICanonicalField` for `EvmKeyIdentifierForm`

`KIND` is `CanonicalFieldKind::FixedBytes32`.

## `EvmKeyIdentifierForm::to_field(params: ToFieldParams, payload: &Self) -> ToFieldReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| yielded | any form | none | none | `Ok(ToFieldSuccessReturn { field: CanonicalFieldValue::FixedBytes32(payload.bytes) })`, the array copied |

`params` carries no control and is not read.

## `EvmKeyIdentifierForm::from_field(params: FromFieldParams, payload: CanonicalFieldValue) -> FromFieldReturn<Self>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| wrong kind | `payload` is not `CanonicalFieldValue::FixedBytes32` | `let CanonicalFieldValue::FixedBytes32(bytes) = payload else { … }` | none | `Err(FromFieldErrorReturn::WrongKind { expected: CanonicalFieldKind::FixedBytes32 })` |
| admitted | `payload` is `CanonicalFieldValue::FixedBytes32(bytes)` | the same match | `EvmKeyIdentifierForm::try_new(EvmKeyIdentifierFormConstructorParams { bytes })`, once, unpacked by `let Ok(value) = …;` | `Ok(FromFieldSuccessReturn { value })` |

## `EvmSignedIntentForm::try_new(params: EvmSignedIntentFormConstructorParams) -> EvmSignedIntentFormTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| construct | any params | none | none | `Ok(EvmSignedIntentForm { bytes })`, the array moved from the params |

The error arm has no branch: `Infallible` is uninhabited.

## `ICanonicalField` for `EvmSignedIntentForm`

`KIND` is `CanonicalFieldKind::FixedBytes32`.

## `EvmSignedIntentForm::to_field(params: ToFieldParams, payload: &Self) -> ToFieldReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| yielded | any form | none | none | `Ok(ToFieldSuccessReturn { field: CanonicalFieldValue::FixedBytes32(payload.bytes) })`, the array copied |

`params` carries no control and is not read.

## `EvmSignedIntentForm::from_field(params: FromFieldParams, payload: CanonicalFieldValue) -> FromFieldReturn<Self>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| wrong kind | `payload` is not `CanonicalFieldValue::FixedBytes32` | `let CanonicalFieldValue::FixedBytes32(bytes) = payload else { … }` | none | `Err(FromFieldErrorReturn::WrongKind { expected: CanonicalFieldKind::FixedBytes32 })` |
| admitted | `payload` is `CanonicalFieldValue::FixedBytes32(bytes)` | the same match | `EvmSignedIntentForm::try_new(EvmSignedIntentFormConstructorParams { bytes })`, once, unpacked by `let Ok(value) = …;` | `Ok(FromFieldSuccessReturn { value })` |

## Ordering and invariants

- Each `from_field` decides by the one match before any construction; the same payload always yields the same outcome; no call has a side effect.
- Each form's `to_field` returns a value of its `KIND`; each form's `from_field` admits every value its `to_field` returns and returns a form equal to the one that yielded it.
- A form's only constructor is its `try_new`, which its own `from_field` calls.

## Integration: own entries and callee dispositions

### Own entries

- **`evm_identity_form_survives_its_field_round_trip`.** Condition: `EvmIdentityForm::to_field` is called over a form and `EvmIdentityForm::from_field` over the field it returns. Outcome: `Ok(FromFieldSuccessReturn { value })` with `value` equal to the original form. Variation: the ascending bytes, which differ from the builder default in every position, so a rebuild from the default, from truncated bytes, or from reordered bytes fails. Edge that must survive the composition: the field `to_field` returns is of the form's `KIND`, which `from_field` admits.
- **`evm_entitlement_form_survives_its_field_round_trip`.** Condition: `EvmEntitlementForm::to_field` is called over a form and `EvmEntitlementForm::from_field` over the field it returns. Outcome: `Ok(FromFieldSuccessReturn { value })` with `value` equal to the original form. Variation: the ascending bytes, which differ from the builder default in every position, so a rebuild from the default, from truncated bytes, or from reordered bytes fails. Edge that must survive the composition: the field `to_field` returns is of the form's `KIND`, which `from_field` admits.
- **`evm_interval_form_survives_its_field_round_trip`.** Condition: `EvmIntervalForm::to_field` is called over a form and `EvmIntervalForm::from_field` over the field it returns. Outcome: `Ok(FromFieldSuccessReturn { value })` with `value` equal to the original form. Variation: the value `0x0102_0304_0506_0708`, which differs from the builder default and whose upper 32 bits are non-zero, so a rebuild from the default or from a value narrowed to 32 bits fails. Edge that must survive the composition: the field `to_field` returns is of the form's `KIND`, which `from_field` admits.
- **`evm_chain_identifier_form_survives_its_field_round_trip`.** Condition: `EvmChainIdentifierForm::to_field` is called over a form and `EvmChainIdentifierForm::from_field` over the field it returns. Outcome: `Ok(FromFieldSuccessReturn { value })` with `value` equal to the original form. Variation: the ascending bytes, which differ from the builder default in every position, so a rebuild from the default, from truncated bytes, or from reordered bytes fails. Edge that must survive the composition: the field `to_field` returns is of the form's `KIND`, which `from_field` admits.
- **`evm_key_identifier_derived_through_the_identity_form_survives_its_field_round_trip`.** Condition: `EvmIdentityForm::derive_key_identifier` is called over a scheme and a key, then `EvmKeyIdentifierForm::to_field` over the key identifier it returns and `EvmKeyIdentifierForm::from_field` over the field. Outcome: `Ok(FromFieldSuccessReturn { value })` with `value` equal to the derived key identifier. Variation: a scheme identifier of `0x5A` and a key of thirty-three ascending bytes, which match no builder default in value or width, so a derivation that ignores the scheme or the key, or a rebuild from a default, fails. Edge that must survive the composition: the field `to_field` returns is of the form's `KIND`, which `from_field` admits, and equals the digest of the declared tag, the scheme byte, and the key.
- **`evm_signed_intent_survives_its_field_round_trip`.** Condition: `EvmForms::signed_intent` is called over the builder's inputs, then `EvmSignedIntentForm::to_field` over the intent it returns and `EvmSignedIntentForm::from_field` over the field. Outcome: `Ok(FromFieldSuccessReturn { value })` with `value` equal to the intent. Variation: inputs that differ from every builder default in each member, so a digest not built from them fails. Edge that must survive the composition: the field `to_field` returns is of the form's `KIND`, which `from_field` admits.
- **`evm_signed_intent_types_its_members_by_the_names_its_encoder_returns`.** Condition: `create_encoding` is called with a selection of the encoding family, its consumer builds `EvmForms::try_new` over the encoder it is handed and calls `signed_intent` over inputs whose call holds one field of each canonical field kind. Outcome: the intent equals the EIP-712 digest assembled from the type names that selection's concrete returns. Variation: the Ethereum ABI selection, whose names are `bytes20`, `uint64`, `bytes32`, `uint16`, `uint32`, `string`, `bytes`, and `uint256`, and the mock selection, whose names are the eight `mock…` names, so a concrete that writes its own names, or reads another encoder's, produces another digest. Edge that must survive the composition: every one of the eight kinds' names reaches the digest, because the call holds one field of each kind and the nonce, expiry, and identity members take theirs from the same encoder.

### Callee dispositions

- `every concrete names every canonical field kind` of `encoding/factory`: absorbed — proven by the `integration_test.rs` block for `evm_signed_intent_types_its_members_by_the_names_its_encoder_returns` run over the mock selection, whose eight distinct names each reach the digest through the call's one field of each kind.
- `every concrete declaring the Ethereum ABI identifier names every kind by its ABI type` of `encoding/factory`: absorbed — proven by the same block run over the Ethereum ABI selection, whose expected digest is assembled from the ABI specification's names `bytes32`, `uint16`, `uint32`, `uint64`, `string`, `bytes`, `bytes20`, and `uint256`, the fixed twenty-byte kind `bytes20` and never `address`.

### Private surface

The chain of real functions is `create_encoding`, the selected encoding concrete's `type_name`, `EvmForms::try_new`, `EvmForms::signed_intent`, `EvmIdentityForm::derive_key_identifier`, and each form's `to_field`, `from_field`, and `try_new`; the outer-edge collaborator is the encoding family's mock concrete under the mock selection, which stands for no real concrete, and the real concrete under the Ethereum ABI selection; the observable result is the form `from_field` returns for the field `to_field` returned and the digest `signed_intent` returns. Every own entry is proven here.
