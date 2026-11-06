# `admit_contexts` — interaction spec

Branch contract for the `admit_contexts` module of `apps/harness-crypto`: `admit_contexts` rebuilds a `CanonicalDeliveryContext` and a purpose, handed as data, into the typed `DeliveryContext<F, Purpose>` the delivery functions and the generate concrete take — the chain identifier, the entitlement contract, the source and target entitlements, the old and new intervals, the seller, and the buyer each rebuilt in transcript order through its form's `from_field`, and the suite identifier, the asset root, the parameter-set identifier, and the expiry carried from the canonical context — and returns a form's refusal whole, the chain identifier's as its own variant and every other's with the `DeliveryContextField` of the field that held the value. Each branch states condition, decision, dependency call, and the exact return outcome.

## `admit_contexts`

`admit_contexts<F: IChainForms, Purpose>(deps: &AdmitContextsDeps, params: AdmitContextsParams, payload: AdmitContextsPayload<'_, Purpose>) -> AdmitContextsReturn<F, Purpose>`, the trusted form: `F` is the function's own type parameter, since the forms it builds are the caller's and the success return names them, and every input is a typed value from the config module.

### Chain identifier refused

Dependency call `<F::ChainIdentifier as ICanonicalField>::from_field(FromFieldParams, payload.context.chain.clone())`; condition a refusal; outcome `Err(AdmitContextsErrorReturn::ChainIdentifier(error))` with `error` unchanged.

### Entitlement contract refused

Dependency call `<F::Identity as ICanonicalField>::from_field(FromFieldParams, payload.context.entitlement_contract.clone())`; condition a refusal; outcome `Err(AdmitContextsErrorReturn::Field { field: DeliveryContextField::EntitlementContract, error })` with `error` unchanged.

### Source entitlement refused

Dependency call `<F::Entitlement as ICanonicalField>::from_field(FromFieldParams, payload.context.source_entitlement.clone())`; condition a refusal; outcome `Err(AdmitContextsErrorReturn::Field { field: DeliveryContextField::SourceEntitlement, error })` with `error` unchanged.

### Target entitlement refused

Dependency call `<F::Entitlement as ICanonicalField>::from_field(FromFieldParams, payload.context.target_entitlement.clone())`; condition a refusal; outcome `Err(AdmitContextsErrorReturn::Field { field: DeliveryContextField::TargetEntitlement, error })` with `error` unchanged.

### Old interval refused

Dependency call `<F::Interval as ICanonicalField>::from_field(FromFieldParams, payload.context.old_interval.clone())`; condition a refusal; outcome `Err(AdmitContextsErrorReturn::Field { field: DeliveryContextField::OldInterval, error })` with `error` unchanged.

### New interval refused

Dependency call `<F::Interval as ICanonicalField>::from_field(FromFieldParams, payload.context.new_interval.clone())`; condition a refusal; outcome `Err(AdmitContextsErrorReturn::Field { field: DeliveryContextField::NewInterval, error })` with `error` unchanged.

### Seller refused

Dependency call `<F::Identity as ICanonicalField>::from_field(FromFieldParams, payload.context.seller.clone())`; condition a refusal; outcome `Err(AdmitContextsErrorReturn::Field { field: DeliveryContextField::Seller, error })` with `error` unchanged.

### Buyer refused

Dependency call `<F::Identity as ICanonicalField>::from_field(FromFieldParams, payload.context.buyer.clone())`; condition a refusal; outcome `Err(AdmitContextsErrorReturn::Field { field: DeliveryContextField::Buyer, error })` with `error` unchanged.

### Success

Condition every form was built; outcome `Ok(AdmitContextsSuccessReturn { context })` with `context` a `DeliveryContext { suite_identifier, chain, entitlement_contract, asset_root, parameter_set_identifier, source_entitlement, target_entitlement, old_interval, new_interval, purpose, seller, buyer, expiry }`, the suite identifier and the parameter-set identifier cloned from the canonical context, the asset root and the expiry copied from it, the purpose moved from the payload, and each form the value its `from_field` returned in `FromFieldSuccessReturn { value }`.

### Ordering

The chain identifier, the entitlement contract, the source entitlement, the target entitlement, the old interval, the new interval, the seller, then the buyer; the first refusal returns and every later form is not built; `deps` and `params` carry nothing and are not read; no side effect.

## Integration: own entries

- `every_chain_forms_concrete_rebuilds_each_form_of_a_context_from_its_canonical_value`: condition `admit_contexts` is called, inside the consumer a concrete of `CHAIN_FORMS_CONCRETES` hands, with the canonical context at its builder defaults, whose chain identifier, entitlement contract, two entitlements, two intervals, seller, and buyer are of the kinds the concrete's forms declare and are pairwise distinct per field; outcome `Ok` whose context holds, in each of the eight form fields, the form whose `to_field` returns the like-named canonical value of the context, and whose suite identifier, asset root, parameter-set identifier, and expiry equal the canonical context's; variation each member of `CHAIN_FORMS_CONCRETES`, paired with the member of `DECLARED_ENCODING_SELECTIONS` its declaration requires; edge that must survive the composition: a value placed in another field's position, the source for the target, the old interval for the new, the seller for the buyer, or the entitlement contract for the seller, a form rebuilt from a default, and a form built by another form's type differ from the value built in Arrange.
- `a_refused_form_names_the_field_that_held_its_value_over_every_chain_forms_concrete`: condition `admit_contexts` is called, inside the consumer a concrete of `CHAIN_FORMS_CONCRETES` hands, with the canonical context at its builder defaults except one form-typed field, which holds a `CanonicalFieldValue::Text`, a kind no form declares; outcome `Err(AdmitContextsErrorReturn::ChainIdentifier(FromFieldErrorReturn::WrongKind { expected }))` for the chain identifier and `Err(AdmitContextsErrorReturn::Field { field, error: FromFieldErrorReturn::WrongKind { expected } })` for each other field, `expected` the `KIND` of the form type the field names; variation each of the eight fields in turn, over each member of `CHAIN_FORMS_CONCRETES`; edge that must survive the composition: a refusal returned with another field's name, with the kind of another form, or as a different variant than the field's differs from the error built in Arrange.

## Integration: callee dispositions

- `chain_forms_identity_survives_its_field_round_trip`, `chain_forms_entitlement_survives_its_field_round_trip`, `chain_forms_interval_survives_its_field_round_trip`, and `chain_forms_chain_identifier_survives_its_field_round_trip` of `chain/create_chain_forms`: absorbed, proven by the `integration_test.rs` block `every_chain_forms_concrete_rebuilds_each_form_of_a_context_from_its_canonical_value`, whose run over `CHAIN_FORMS_CONCRETES` reaches `F::Identity`, `F::Entitlement`, `F::Interval`, and `F::ChainIdentifier` through `admit_contexts` and reads each admitted form back through `to_field`.
- The remaining entries of `chain/create_chain_forms` concern the declaration the forms hand a consumer, the encoder admission, the key identifier, and the signed intent, none of which `admit_contexts` calls; the entries of `encoding/factory` concern the encoding concretes' handing to a consumer, those of `proof/factory` the admission of a delivery-proof concrete, those of `harness-crypto/generate/solidity/challenge_vectors` the challenge records, and those of `harness-crypto/config/admit_statements` the admission of the document's text, and no entry of them passes through `admit_contexts` or ends at its tests.

## Integration: private surface

The chain of real functions `create_chain_forms_case`, `create_encoding` with the encoding concrete the case params select, `create_chain_forms`, the selected forms concrete's producer, `consume_chain_forms`, `admit_contexts`, and each form's `from_field` and `to_field`, all running for real over the real declared sets; the outer-edge collaborator is the encoding family's real Ethereum ABI concrete; the observable result is the in-place assertions of the consumer on the `AdmitContextsReturn`; the entries proven are `every_chain_forms_concrete_rebuilds_each_form_of_a_context_from_its_canonical_value` and `a_refused_form_names_the_field_that_held_its_value_over_every_chain_forms_concrete`.

## Integration: public surface

An outside caller invokes `admit_contexts` through the crate root with an `AdmitContextsDeps`, an `AdmitContextsParams`, and an `AdmitContextsPayload` holding a `CanonicalDeliveryContext` and a purpose, from inside the `IChainFormsConsumer` it hands `chain`'s `create_chain_forms_case` through the chain crate's public surface under the `mocks` feature, and observes the `AdmitContextsReturn`, the forms in its success through `ICanonicalField::to_field`, and its refusal; the entries proven are `every_chain_forms_concrete_rebuilds_each_form_of_a_context_from_its_canonical_value` and `a_refused_form_names_the_field_that_held_its_value_over_every_chain_forms_concrete`; the outer-edge collaborator is the encoding family's real Ethereum ABI concrete.
