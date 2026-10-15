# `create_chain_forms` — interaction spec

Branch contract for the `create_chain_forms` module of the `chain` crate: the family's forms construction point — it selects the forms concrete `params.concrete` names, admits it only when its declared identifier is `params.identifier`, constructs it, and hands it to the consumer in `deps`, which is generic over `IChainForms` and never names the concrete. Each branch states condition, decision, dependency call, and the exact return outcome.

## `create_chain_forms<C: IChainFormsConsumer>(deps: &CreateChainFormsDeps<C>, params: CreateChainFormsParams, payload: CreateChainFormsPayload) -> CreateChainFormsReturn<C::Output>`

Decision: a `match` on `params.concrete`, one arm per `ChainFormsConcrete` variant, exhaustive so a variant with no arm fails to compile.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| unsupported identifier | the named concrete's `DECLARATION.identifier` is not `params.identifier` | equality, read before any construction | none | `Err(CreateChainFormsErrorReturn::UnsupportedChainFormsIdentifier)`; nothing is constructed and the consumer is not called |
| admitted | the named concrete's `DECLARATION.identifier` is `params.identifier` | the same comparison | `EvmForms::try_new(EvmFormsConstructorParams)`, exactly once, its success destructured irrefutably because its error arm is uninhabited; then `deps.consumer.consume_chain_forms(ConsumeChainFormsParams, ConsumeChainFormsPayload { forms })`, exactly once | `Ok(CreateChainFormsSuccessReturn { output })`, holding the consumer's output |

`Mock` arm, under `#[cfg(any(test, feature = "mocks"))]`: condition `params.concrete` is `Mock`; decision `MockIChainForms::DECLARATION.identifier` equals `params.identifier`, read before any construction; dependency calls `build_mock_i_chain_forms(MockIChainFormsOverrides)`, which has no error arm, then `deps.consumer.consume_chain_forms(ConsumeChainFormsParams, ConsumeChainFormsPayload { forms })`, exactly once; outcome `Ok(CreateChainFormsSuccessReturn { output })` holding the consumer's output; the unequal-identifier outcome is the unsupported-identifier outcome.

`ChainFormsIdentifier` has the one variant the EVM forms concrete declares, so no input takes the unsupported-identifier branch until a further identifier exists; the branch has no unit test.

`CreateChainFormsErrorReturn::Evm` carries the EVM concrete's uninhabited constructor error type in the return union, so no branch produces it.

## Ordering and invariants

- Within an arm: the identifier check, then construction, then the consumer. `params.concrete` selects the concrete; `params.identifier` admits it.
- `payload` carries nothing and is not read. `deps.consumer` is invoked at most once per call, only on the admitted branch.
- The consumer receives the forms as `F: IChainForms` and reads `F::DECLARATION`; nothing in this module names a forms concrete other than in the selecting arm.

## Integration: callee dispositions

The entries of `chain/evm_forms` that this node's routes reach, each dispositioned as stated.

- `evm_identity_form_survives_its_field_round_trip` (`chain/evm_forms`): absorbed; proven by the `integration_test.rs` block for `chain_forms_identity_survives_its_field_round_trip`, whose run over `CHAIN_FORMS_CONCRETES` reaches `EvmIdentityForm` through `IChainForms::Identity`.
- `evm_entitlement_form_survives_its_field_round_trip` (`chain/evm_forms`): absorbed; proven by the `integration_test.rs` block for `chain_forms_entitlement_survives_its_field_round_trip`, whose run over `CHAIN_FORMS_CONCRETES` reaches `EvmEntitlementForm` through `IChainForms::Entitlement`.
- `evm_interval_form_survives_its_field_round_trip` (`chain/evm_forms`): absorbed; proven by the `integration_test.rs` block for `chain_forms_interval_survives_its_field_round_trip`, whose run over `CHAIN_FORMS_CONCRETES` reaches `EvmIntervalForm` through `IChainForms::Interval`.
- `evm_chain_identifier_form_survives_its_field_round_trip` (`chain/evm_forms`): absorbed; proven by the `integration_test.rs` block for `chain_forms_chain_identifier_survives_its_field_round_trip`, whose run over `CHAIN_FORMS_CONCRETES` reaches `EvmChainIdentifierForm` through `IChainForms::ChainIdentifier`.

## Integration: own entries

Each entry is proven by the private integration block of the same name and by the public integration test over `CHAIN_FORMS_CONCRETES`.

- `chain_forms_handed_to_the_consumer_declare_the_required_identifier`: condition `create_chain_forms` is called with a consumer whose method returns the `F::DECLARATION` of the forms it receives, a selection, and `params.identifier`; outcome `Ok(CreateChainFormsSuccessReturn { output })` with `output` the declaration the consumer read from the forms it received, whose `identifier` equals `params.identifier` and whose `interface_version` equals `CHAIN_FORMS_INTERFACE_VERSION`; variation each concrete of `CHAIN_FORMS_CONCRETES` and the `Mock` selection, so a function that returns anything other than the consumer's output fails; edge that must survive the composition: the concrete is constructed before the consumer runs, and the consumer's output reaches the return unaltered.
- `chain_forms_identity_survives_its_field_round_trip`: condition a consumer builds `F::Identity` by `from_field` over `CanonicalFieldValue::FixedBytes20` holding the ascending bytes and yields the field back by `to_field` over the form it built; outcome the field `to_field` returns equals the field `from_field` was given, and `F::Identity::KIND` is `CanonicalFieldKind::FixedBytes20`; variation each concrete of `CHAIN_FORMS_CONCRETES` and the `Mock` selection, with the ascending bytes, position `i` holding `i + 1`, which differ as a whole from each concrete's builder default and hold no repeated byte, so a rebuild from a default, a constant, truncated bytes, or reordered bytes fails; edge that must survive the composition: the field the form yields is of the form's `KIND`, which its `from_field` admits.
- `chain_forms_entitlement_survives_its_field_round_trip`: condition a consumer builds `F::Entitlement` by `from_field` over `CanonicalFieldValue::Unsigned256` holding the ascending bytes and yields the field back by `to_field` over the form it built; outcome the field `to_field` returns equals the field `from_field` was given, and `F::Entitlement::KIND` is `CanonicalFieldKind::Unsigned256`; variation each concrete of `CHAIN_FORMS_CONCRETES` and the `Mock` selection, with the ascending bytes, which differ as a whole from each concrete's builder default and hold no repeated byte, so a rebuild from a default, a constant, truncated bytes, or reordered bytes fails; edge that must survive the composition: the field the form yields is of the form's `KIND`, which its `from_field` admits.
- `chain_forms_interval_survives_its_field_round_trip`: condition a consumer builds `F::Interval` by `from_field` over `CanonicalFieldValue::Unsigned64` holding `0x0102_0304_0506_0708` and yields the field back by `to_field` over the form it built; outcome the field `to_field` returns equals the field `from_field` was given, and `F::Interval::KIND` is `CanonicalFieldKind::Unsigned64`; variation each concrete of `CHAIN_FORMS_CONCRETES` and the `Mock` selection, with the value `0x0102_0304_0506_0708`, which differs from each concrete's builder default and whose upper 32 bits are non-zero, so a rebuild from a default or from a value narrowed to 32 bits fails; edge that must survive the composition: the field the form yields is of the form's `KIND`, which its `from_field` admits.
- `chain_forms_chain_identifier_survives_its_field_round_trip`: condition a consumer builds `F::ChainIdentifier` by `from_field` over `CanonicalFieldValue::Unsigned256` holding the ascending bytes and yields the field back by `to_field` over the form it built; outcome the field `to_field` returns equals the field `from_field` was given, and `F::ChainIdentifier::KIND` is `CanonicalFieldKind::Unsigned256`; variation each concrete of `CHAIN_FORMS_CONCRETES` and the `Mock` selection, with the ascending bytes, which differ as a whole from each concrete's builder default and hold no repeated byte, so a rebuild from a default, a constant, truncated bytes, or reordered bytes fails; edge that must survive the composition: the field the form yields is of the form's `KIND`, which its `from_field` admits.

## Integration: private surface

The chain of real functions is `create_chain_forms`, the selected concrete's producer (`EvmForms::try_new` for `Evm`, `build_mock_i_chain_forms` for `Mock`), `consume_chain_forms`, and each form's `from_field`, `to_field`, and `try_new`; no outer-edge collaborator is mocked, the `Mock` selection standing for no real concrete; the observable result is the declaration the consumer returns for the declaration entry, and the call's `Ok` return with the consumer's in-place assertions for each round-trip entry.

## Integration: public surface

An outside caller invokes `create_chain_forms` with its own `IChainFormsConsumer`, a concrete from `CHAIN_FORMS_CONCRETES`, and the identifier; it observes the consumer's output in the return and, inside the consumer, the declaration and each form's `from_field` and `to_field` results through `IChainForms` and `ICanonicalField`; the entries proven are `chain_forms_handed_to_the_consumer_declare_the_required_identifier`, `chain_forms_identity_survives_its_field_round_trip`, `chain_forms_entitlement_survives_its_field_round_trip`, `chain_forms_interval_survives_its_field_round_trip`, and `chain_forms_chain_identifier_survives_its_field_round_trip`, each over `CHAIN_FORMS_CONCRETES`, along the route `create_chain_forms`, the concrete's producer, `consume_chain_forms`, and each form's `from_field`, `to_field`, and `try_new`, with no outer-edge collaborator.

