# admit_hash_to_scalar_selection interactions

`admit_hash_to_scalar_selection` is the hash-to-scalar family's selection admission: it receives the admitted member identifier `params.concrete` and the untrusted settings document, refuses a document that supplies any setting with the family's own error naming the member, and otherwise returns the admitted member identifier unchanged as the selection. No member of the hash-to-scalar family has a settable value, so a supplied setting would carry configuration no member reads, and the admission refuses it instead of returning a selection that silently discards it.

The document decision is computed once on `payload`, before the match, and is shared by every member's arm: the document supplies no setting when it is `Value::Null` or a `Value::Object` whose map is empty, and supplies a setting when it is any other `serde_json::Value`.

## `admit_hash_to_scalar_selection`

`admit_hash_to_scalar_selection(deps: &AdmitHashToScalarSelectionDeps, params: AdmitHashToScalarSelectionParams, payload: serde_json::Value) -> AdmitHashToScalarSelectionReturn`

The decision is an exhaustive `match` on `params.concrete`, one arm per `HashToScalarConcrete` variant, so a variant with no arm fails to compile and a further concrete adds its own arm; each arm realizes the two branches below, in the order stated.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| keccak256, document supplies a setting | `params.concrete` is `HashToScalarConcrete::Keccak256` and the document decision finds a setting — `payload` is neither `Value::Null` nor a `Value::Object` that is empty, `payload.is_null()` and `payload.as_object().is_some_and(\|map\| map.is_empty())` both false | the exhaustive `match` on `params.concrete` and the document decision | none | `Err(AdmitHashToScalarSelectionErrorReturn::SettingSupplied { concrete: params.concrete })` |
| keccak256, document supplies no setting | `params.concrete` is `HashToScalarConcrete::Keccak256` and the document decision finds no setting — the payload is `Value::Null` or an empty `Value::Object` | the same `match` and document decision | none | `Ok(AdmitHashToScalarSelectionSuccessReturn { selection: params.concrete })`, the admitted member identifier returned unchanged |
| mock, document supplies a setting — present only under `#[cfg(any(test, feature = "mocks"))]` | `params.concrete` is `HashToScalarConcrete::Mock(mode)` and the document decision finds a setting by the same decision | the same `match` and document decision | none | `Err(AdmitHashToScalarSelectionErrorReturn::SettingSupplied { concrete: params.concrete })` |
| mock, document supplies no setting — present only under `#[cfg(any(test, feature = "mocks"))]` | `params.concrete` is `HashToScalarConcrete::Mock(mode)` and the document decision finds no setting | the same `match` and document decision | none | `Ok(AdmitHashToScalarSelectionSuccessReturn { selection: params.concrete })`, the selected mock member returned with the mode it was selected with |

Ordering and side effects: the document decision is computed once before the `match`; the function constructs no adapter, reads no declaration, calls no collaborator, and touches no operating-system facility; the same params and document always yield the same outcome.

## Integration: own entries

- `every_declared_member_is_admitted_to_the_selection_that_names_it`: condition the admission is called with each of `Value::Null` and an empty `Value::Object`, and each member of `HASH_TO_SCALAR_CONCRETES` as `params.concrete`; outcome `Ok` whose `selection` equals that member; variation every member of the declared set and each admitted document form; edge that must survive: the admitted member identifier reaches the caller equal to the identifier given, with no adapter constructed and no declaration read.
- `a_mock_member_is_admitted_to_the_selection_naming_the_selected_mock`: condition the same call under `#[cfg(any(test, feature = "mocks"))]` with `params.concrete` `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::SamplingRefused)` and an empty `Value::Object`; outcome `Ok` whose `selection` equals that member; variation `SamplingRefused` against the `Succeeds` mode the mock constant names, so a selection that substitutes a default mock member differs from one that carries the selection, and an empty object against `Value::Null`; edge that must survive: the selected mock member reaches the caller carrying the mode selected.
- `a_setting_supplied_for_a_member_is_refused_naming_the_member`: condition the same call with each of a `Value::Object` holding one key and `Value::from(true)`, for each member of `HASH_TO_SCALAR_CONCRETES` and, under the same `cfg`, for the `SamplingRefused` mock member; outcome `Err(AdmitHashToScalarSelectionErrorReturn::SettingSupplied { concrete })` with `concrete` that member; variation an object holding a key and a non-object against the empty object and `Value::Null` that are admitted; edge that must survive: the refusal names the member given.

## Integration: callee dispositions

None; the function calls no collaborator, so no entry of another node passes through it or ends at one of its tests.

## Integration: private surface

None; the function composes no collaborator, so it carries no private integration test and the module declares no `integration_test.rs`.

## Integration: public surface

An outside caller invokes `admit_hash_to_scalar_selection` with the fieldless `AdmitHashToScalarSelectionDeps`, `AdmitHashToScalarSelectionParams { concrete }`, and a `serde_json::Value` document written directly, and observes the returned `Result`; the entries the public integration test proves are `every_declared_member_is_admitted_to_the_selection_that_names_it`, `a_mock_member_is_admitted_to_the_selection_naming_the_selected_mock`, and `a_setting_supplied_for_a_member_is_refused_naming_the_member`.
