# `admit_random_selection` — interaction spec

Branch contract for the `admit_random_selection` module of the `random` crate: the randomness family's selection admission — receives the admitted member identifier and the untrusted settings document, refuses a document that supplies any setting with the family's own error naming the member, since no member has a settable value, and returns the selection naming the member. Each branch states condition, decision, dependency call, and the exact return outcome.

## `admit_random_selection(deps: &AdmitRandomSelectionDeps, params: AdmitRandomSelectionParams, payload: serde_json::Value) -> AdmitRandomSelectionReturn`

The validating form: `deps` is not read. Decision: an exhaustive `match` on `params.identifier`, one arm per `RandomSourceKind` variant, so a further member adds an arm; in every arm the document supplies no setting when the payload is `Value::Null` or a `Value::Object` that holds no entry, and supplies a setting for every other payload.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| operating system — no setting supplied | `params.identifier` is `RandomSourceKind::OperatingSystem` and the payload is `Value::Null` or an empty `Value::Object` | the document supplies no setting | none | `Ok(AdmitRandomSelectionSuccessReturn { selection: RandomSourceKind::OperatingSystem })` |
| operating system — a setting supplied | `params.identifier` is `RandomSourceKind::OperatingSystem` and the payload is a `Value::Object` that holds an entry, or a `Value::Bool`, `Value::Number`, `Value::String`, or `Value::Array` | the document supplies a setting and the member has none | none | `Err(AdmitRandomSelectionErrorReturn::SettingsSupplied { identifier: RandomSourceKind::OperatingSystem })` |
| mock, no setting supplied — `#[cfg(any(test, feature = "mocks"))]` | `params.identifier` is `RandomSourceKind::Mock(mode)` and the payload is `Value::Null` or an empty `Value::Object` | the document supplies no setting | none | `Ok(AdmitRandomSelectionSuccessReturn { selection: RandomSourceKind::Mock(mode) })`, the selection carrying the failure mode it was given |
| mock, a setting supplied — `#[cfg(any(test, feature = "mocks"))]` | `params.identifier` is `RandomSourceKind::Mock(mode)` and the payload supplies a setting | the document supplies a setting and the member has none | none | `Err(AdmitRandomSelectionErrorReturn::SettingsSupplied { identifier: RandomSourceKind::Mock(mode) })` |

## Ordering and invariants

- The matched branch returns; no side effect — no clock, no environment variable, no network, no filesystem; the payload is inspected and not retained.

## Integration: callee dispositions

No entry of `random/os` or of `random/lookup_random_declaration` passes through `admit_random_selection` or ends at one of its tests, since the function calls none of their functions and names none of their items.

## Integration: own entries

- `every_declared_member_is_admitted_to_the_selection_naming_it`: condition `admit_random_selection` is called by a caller outside the crate over each identifier of `RANDOM_SOURCE_KINDS`, with a document that is `Value::Null` and with a document that is an empty `Value::Object`; outcome `Ok` whose selection equals the identifier; variation the documents that supply no setting, `Value::Null` and an empty `Value::Object`, so a check that refuses either fails; edge that must survive the composition: the selection reaches the outside caller through this function alone, and the declared set holds no mock member.
- `the_mock_member_is_admitted_to_the_selection_carrying_its_failure_mode`: condition `admit_random_selection` is called by a caller outside the crate under the `mocks` feature over `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)` and over `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::Succeeds)`, with a document that is `Value::Null` and with a document that is an empty `Value::Object`; outcome `Ok` whose selection is the identifier looked at, its failure mode included; variation the failure modes `FillBytesRefused` and `Succeeds`, and the documents `Value::Null` and an empty `Value::Object`, so a branch that returns one fixed mode, or the real member's selection, fails; edge that must survive the composition: the mock selection keeps the failure mode its configuration names.
- `a_document_supplying_a_setting_is_refused_naming_the_member`: condition `admit_random_selection` is called by a caller outside the crate over each identifier of `RANDOM_SOURCE_KINDS` and, under the `mocks` feature, over `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)`, with a document that is a `Value::Object` holding an entry, with a document that is an empty `Value::Array`, with a document that is a `Value::Bool`, with a document that is a `Value::Number`, and with a document that is a `Value::String`; outcome `Err(AdmitRandomSelectionErrorReturn::SettingsSupplied { identifier })` naming the identifier looked at; variation the object holding an entry, the empty array, a boolean, a number, and a string, where the empty array is a value an emptiness check on the document would admit, so a check that refuses only objects, or that omits one of these kinds, fails; edge that must survive the composition: the refusal names the member, the mock member's failure mode included.

## Integration: private surface

The real function `admit_random_selection`, which calls no function; the observable result is the `AdmitRandomSelectionReturn`; the branches are proven by the unit blocks of `test.rs`, and the node carries no private integration test.

## Integration: public surface

The crate's `lib.rs` re-exports `admit_random_selection::provides`, so an outside caller invokes `admit_random_selection` over a member identifier and a settings document. It observes the `AdmitRandomSelectionReturn`: the `Ok` arm's selection and the `Err` arm's identifier through their public fields. The entries proven are `every_declared_member_is_admitted_to_the_selection_naming_it`, `the_mock_member_is_admitted_to_the_selection_carrying_its_failure_mode`, and `a_document_supplying_a_setting_is_refused_naming_the_member`. The route is `admit_random_selection`, with no outer-edge collaborator.
