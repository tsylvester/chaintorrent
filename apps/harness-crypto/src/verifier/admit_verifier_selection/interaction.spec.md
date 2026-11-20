# `admit_verifier_selection` — interaction spec

Branch contract for the `admit_verifier_selection` module of the `verifier` module of `apps/harness-crypto`: `admit_verifier_selection` selects the verifier concrete the configuration names, narrowing the selected member's untrusted settings document, the member's own table, through that member's guard, reading that member's declaration through the declaration lookup its deps carry, and admitting the document against that declaration through the member's settings admission its deps carry — a guard refusal returning in the member's document variant with the callee's error unchanged — producing the settings-carrying `VerifierConcrete`. Each function's branches state condition, decision, dependency call, and the exact return outcome.

## `admit_verifier_selection`

`admit_verifier_selection(deps: &AdmitVerifierSelectionDeps, params: AdmitVerifierSelectionParams, payload: serde_json::Value) -> AdmitVerifierSelectionReturn`, the validating form; `deps` supplies the declaration lookup and the EVM member's settings admission as handles, each called only through its field; the decision is an exhaustive `match` on `params.identifier`, so a further member adds an arm.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| EVM, narrowing | `params.identifier` is `VerifierConcreteKind::Evm` | the guard `EvmVerifierSettingsDocument::try_from(payload)`, the first statement of the arm | `EvmVerifierSettingsDocument::try_from` | a refusal `Err(AdmitVerifierSelectionErrorReturn::EvmDocument(error))` with the error unchanged; else the document passes to the next row |
| EVM, declaration read | the guard narrowed the payload | none | `(deps.lookup_verifier_declaration)(&LookupVerifierDeclarationDeps, LookupVerifierDeclarationParams { identifier: params.identifier }, LookupVerifierDeclarationPayload)`, bound through the irrefutable pattern `let Ok(looked_up) = …;` because its error arm is `Infallible` | the member's declaration is `looked_up.declaration` |
| EVM, admission | the declaration read | none | `(deps.admit_evm_verifier_settings)(&AdmitEvmVerifierSettingsDeps { declaration: &looked_up.declaration }, AdmitEvmVerifierSettingsParams, AdmitEvmVerifierSettingsPayload { document })`, its result unpacked irrefutably because its error arm is `Infallible` | `Ok(AdmitVerifierSelectionSuccessReturn { selection: VerifierConcrete::Evm { settings: Box::new(admitted.settings) } })` |
| Mock, under `#[cfg(any(test, feature = "mocks"))]` | `params.identifier` is `VerifierConcreteKind::Mock` | none; `payload` is not read, the mock concrete holding no settings | none; no handle is read | `Ok(AdmitVerifierSelectionSuccessReturn { selection: VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure) })` |

### Ordering

The guard, then the declaration read, then the admission; a guard refusal returns before the admission is called; no side effect. The admission has no side effect and its error arm is uninhabited, so no input and no failure mode of the official mocks distinguishes this order from another, and no block proves it.

## Integration: callee dispositions

- `harness-crypto/verifier/lookup_verifier_declaration`: its spec declares no own entry and carries no private integration test, so no entry passes through a function of this node or ends at one of its tests; this node hands the declaration the handle returns to the admission and asserts no value read from it.
- `harness-crypto/verifier/evm/settings`: the node's spec declares no own entry and no private surface, so no entry passes through a function of this node or ends at one of its tests.
- `harness-crypto/verifier/evm`: no entry of its interaction spec passes through a function of this node or ends at one of its tests, this node reading the family's selection types and calling none of the adapter's functions.

## Integration: own entries

- `a_payload_that_is_not_a_table_is_refused_by_the_guard_in_the_document_variant`: condition the call over `VerifierConcreteKind::Evm` with a payload that is not an object; outcome `Err(AdmitVerifierSelectionErrorReturn::EvmDocument(EvmVerifierSettingsDocumentErrorReturn::NotATable { path }))` with the guard's error unchanged; variation a value of the wrong kind, so a payload narrowed by anything weaker than the guard fails; edge that must survive the composition: the guard's error arrives whole in the document variant.
- `a_key_the_member_does_not_declare_is_refused_by_the_guard_in_the_document_variant`: condition the call over `VerifierConcreteKind::Evm` with an object holding a key; outcome `Err(AdmitVerifierSelectionErrorReturn::EvmDocument(EvmVerifierSettingsDocumentErrorReturn::UnknownKey { path, key }))` naming that key; variation a non-empty object, so a payload narrowed by a guard that admits any object fails; edge that must survive the composition: the path and key arrive unchanged.
- `the_admitted_settings_are_carried_in_the_evm_selection`: condition the call over `VerifierConcreteKind::Evm` with a payload the guard narrows while the deps' admission handle returns admitted settings; outcome `Ok` whose selection is `VerifierConcrete::Evm` holding those settings boxed; variation the Evm variant against the Mock variant, so a selection built in another variant, or over settings other than the admission's, fails; edge: the settings the admission returned reach the selection unchanged.
- `every_declared_member_is_selected_at_its_declared_defaults_by_the_real_chain`: condition `admit_verifier_selection` is called over each member of `VERIFIER_CONCRETES` with a payload holding no key; outcome `Ok` whose selection equals the declared-selection builder's selection for that member; variation every member of the declared set, so a branch that returns another member's settings fails; edge that must survive the composition: the selection is the value `build_verifier_concrete` returns for that member.

## Integration: private surface

The chain of real functions `admit_verifier_selection`, `EvmVerifierSettingsDocument::try_from`, `lookup_verifier_declaration`, and `admit_evm_verifier_settings`, reached through `ADMIT_VERIFIER_SELECTION_DEPS`; no outer-edge collaborator is mocked; the observable result is the `AdmitVerifierSelectionReturn`; the private integration test proves `every_declared_member_is_selected_at_its_declared_defaults_by_the_real_chain`.

The unit blocks of `test.rs` prove every branch, with the guard running and the lookup and the admission supplied as handles: `a_payload_that_is_not_a_table_is_refused_by_the_guard_in_the_document_variant`, `a_key_the_member_does_not_declare_is_refused_by_the_guard_in_the_document_variant`, `the_admitted_settings_are_carried_in_the_evm_selection`, and `the_mock_member_selects_the_mock_concrete_under_no_failure_without_reading_the_payload`.

## Integration: public surface

None; the crate's `lib.rs` re-exports `verifier/provides.rs`, which re-exports this module's `provides.rs`, whose types are public and whose function is `pub(crate)`, so no outside call reaches the function and no outcome of this node is observable through the public surface; the node carries no public integration test.
