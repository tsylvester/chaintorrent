# `evm_forms` — interaction spec

Branch contract for the `evm_forms` module of the `chain` crate: the EVM suite's forms as the chain family's first concrete — the identity form over twenty bytes, the entitlement and chain-identifier forms over 256-bit unsigned words, and the interval form over a 64-bit unsigned value, each implementing `ICanonicalField`, and `EvmForms` implementing `IChainForms` over them. Each branch states condition, decision, dependency call, and the exact return outcome.

## `EvmForms::try_new(params: EvmFormsConstructorParams) -> EvmFormsTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| construct | any params | none | none | `Ok(EvmForms)` |

The error arm has no branch: the concrete takes no configuration, so `Infallible` is uninhabited.

## `EvmForms::DECLARATION`

An inherent constant, readable from the type before any instance exists:

`ChainFormsDeclaration { identifier: ChainFormsIdentifier::EvmV1, adapter_version: 1, interface_version: CHAIN_FORMS_INTERFACE_VERSION }`

## `IChainForms` for `EvmForms`

`const DECLARATION: ChainFormsDeclaration = EvmForms::DECLARATION;` `Identity` is `EvmIdentityForm`, `Entitlement` is `EvmEntitlementForm`, `Interval` is `EvmIntervalForm`, and `ChainIdentifier` is `EvmChainIdentifierForm`.

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

## Ordering and invariants

- Each `from_field` decides by the one match before any construction; the same payload always yields the same outcome; no call has a side effect.
- Each form's `to_field` returns a value of its `KIND`; each form's `from_field` admits every value its `to_field` returns and returns a form equal to the one that yielded it.
- A form's only constructor is its `try_new`, which its own `from_field` calls.

## Integration: own entries

- `evm_identity_form_survives_its_field_round_trip`: condition `EvmIdentityForm::to_field` is called over a form and `EvmIdentityForm::from_field` over the field it returns; outcome `Ok(FromFieldSuccessReturn { value })` with `value` equal to the original form; variation the ascending bytes, which differ from the builder default in every position, so a rebuild from the default, from truncated bytes, or from reordered bytes fails; edge that must survive the composition: the field `to_field` returns is of the form's `KIND`, which `from_field` admits.
- `evm_entitlement_form_survives_its_field_round_trip`: condition `EvmEntitlementForm::to_field` is called over a form and `EvmEntitlementForm::from_field` over the field it returns; outcome `Ok(FromFieldSuccessReturn { value })` with `value` equal to the original form; variation the ascending bytes, which differ from the builder default in every position, so a rebuild from the default, from truncated bytes, or from reordered bytes fails; edge that must survive the composition: the field `to_field` returns is of the form's `KIND`, which `from_field` admits.
- `evm_interval_form_survives_its_field_round_trip`: condition `EvmIntervalForm::to_field` is called over a form and `EvmIntervalForm::from_field` over the field it returns; outcome `Ok(FromFieldSuccessReturn { value })` with `value` equal to the original form; variation the value `0x0102_0304_0506_0708`, which differs from the builder default and whose upper 32 bits are non-zero, so a rebuild from the default or from a value narrowed to 32 bits fails; edge that must survive the composition: the field `to_field` returns is of the form's `KIND`, which `from_field` admits.
- `evm_chain_identifier_form_survives_its_field_round_trip`: condition `EvmChainIdentifierForm::to_field` is called over a form and `EvmChainIdentifierForm::from_field` over the field it returns; outcome `Ok(FromFieldSuccessReturn { value })` with `value` equal to the original form; variation the ascending bytes, which differ from the builder default in every position, so a rebuild from the default, from truncated bytes, or from reordered bytes fails; edge that must survive the composition: the field `to_field` returns is of the form's `KIND`, which `from_field` admits.

## Integration: private surface

The chain of real functions is each form's `to_field`, `from_field`, and `try_new`; no outer-edge collaborator is mocked; the observable result is the form `from_field` returns for the field `to_field` returned.

