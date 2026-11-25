# admit_hash_to_scalar_selection interactions

`admit_hash_to_scalar_selection` is the hash-to-scalar family's selection admission: it receives the admitted member identifier `params.concrete` and the untrusted settings document, refuses a document that supplies any setting with the family's own error naming the member, and otherwise returns the selection naming the member the declaration lookup reads. No member of the hash-to-scalar family has a settable value, so a supplied setting would carry configuration no member reads, and the admission refuses it instead of returning a selection that silently discards it.

The document decision is shared by every member's arm: the document supplies no setting when it is `Value::Null` or a `Value::Object` whose map is empty, and supplies a setting when it is any other value.

## `admit_hash_to_scalar_selection`

`admit_hash_to_scalar_selection(deps: &AdmitHashToScalarSelectionDeps, params: AdmitHashToScalarSelectionParams, payload: serde_json::Value) -> AdmitHashToScalarSelectionReturn`

The decision is an exhaustive `match` on `params.concrete`, one arm per `HashToScalarConcrete` variant, so a variant with no arm fails to compile and a further concrete adds its own arm; each arm realizes the two branches below, in the order stated.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| keccak256, document supplies a setting | `params.concrete` is `HashToScalarConcrete::Keccak256` and the document decision finds a setting — `payload.is_null()` and `payload.as_object().is_some_and(\|map\| map.is_empty())` both false | the exhaustive `match` on `params.concrete` and the document decision | none | `Err(AdmitHashToScalarSelectionErrorReturn::SettingSupplied { concrete: params.concrete })` |
| keccak256, document supplies no setting | `params.concrete` is `HashToScalarConcrete::Keccak256` and the document decision finds no setting — the payload is `Value::Null` or an empty `Value::Object` | the same `match` and document decision | `(deps.lookup_declaration)(&LookupHashToScalarDeclarationDeps, LookupHashToScalarDeclarationParams { concrete: params.concrete }, LookupHashToScalarDeclarationPayload)` once, its result bound by `let Ok(success) = result;` | `Ok(AdmitHashToScalarSelectionSuccessReturn { selection: success.declaration.concrete })` |
| mock, document supplies a setting — present only under `#[cfg(any(test, feature = "mocks"))]` | `params.concrete` is `HashToScalarConcrete::Mock(mode)` and the document decision finds a setting by the same decision | the same `match` and document decision | none | `Err(AdmitHashToScalarSelectionErrorReturn::SettingSupplied { concrete: params.concrete })` |
| mock, document supplies no setting — present only under `#[cfg(any(test, feature = "mocks"))]` | `params.concrete` is `HashToScalarConcrete::Mock(mode)` and the document decision finds no setting | the same `match` and document decision | the lookup once with `concrete: params.concrete` | `Ok(AdmitHashToScalarSelectionSuccessReturn { selection: success.declaration.concrete })` |

Ordering and side effects: the function constructs no adapter and touches no operating-system facility; the lookup returns no error and has no effect, so whether it is called on a refusal branch is not observable and no block proves it; the lookup's error arm has no variant, so no callee error is carried.

## Integration: own entries

- `every_declared_member_is_admitted_to_the_selection_that_names_it`: condition the admission is called with the real lookup function, each of `Value::Null` and an empty `Value::Object`, and each member of `HASH_TO_SCALAR_CONCRETES` as `params.concrete`; outcome `Ok` whose `selection` equals that member; variation every member of the declared set and each admitted document form; edge that must survive: the selection the declaration names reaches the caller equal to the identifier given, with no adapter constructed.
- `a_mock_member_is_admitted_to_the_selection_naming_the_selected_mock`: condition the same call under `#[cfg(any(test, feature = "mocks"))]` with `params.concrete` `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::SamplingRefused)` and an empty `Value::Object`; outcome `Ok` whose `selection` equals that member; variation `SamplingRefused` against the `Succeeds` mode the mock declaration constant names, and an empty object against `Value::Null`; edge that must survive: the substituted mock identifier reaches the caller, so a lookup that returned the constant unchanged yields the `Succeeds` member.
- `a_setting_supplied_for_a_member_is_refused_naming_the_member`: condition the same call with each of a `Value::Object` holding one key and `Value::from(true)`, for each member of `HASH_TO_SCALAR_CONCRETES` and, under the same `cfg`, for the `SamplingRefused` mock member; outcome `Err(AdmitHashToScalarSelectionErrorReturn::SettingSupplied { concrete })` with `concrete` that member; variation an object holding a key and a non-object against the empty object and `Value::Null` that are admitted; edge that must survive: the refusal names the member given.

## Integration: callee dispositions

- `lookup_hash_to_scalar_declaration` entry `every_declared_member_declares_itself_through_the_public_surface`: carried, restated in this node's terms as: the declaration read for each member of `HASH_TO_SCALAR_CONCRETES` with no adapter constructed names that member, and reaches the admission's caller as the selection; route: the crate barrel `lib.rs`, `admit_hash_to_scalar_selection::provides`, `admit_hash_to_scalar_selection`, `lookup_hash_to_scalar_declaration`, then the member's interface-held declaration constant; the route reaches no outer-edge collaborator.
- `lookup_hash_to_scalar_declaration` entry `a_mock_member_declares_the_member_selected_through_the_public_surface`: carried, restated in this node's terms as: the declaration the lookup returns for `HashToScalarConcrete::Mock(mode)` carries the mode selected, and reaches the admission's caller as the selection; route: the crate barrel `lib.rs`, `admit_hash_to_scalar_selection::provides`, `admit_hash_to_scalar_selection`, `lookup_hash_to_scalar_declaration`, then the mock declaration constant; the route reaches no outer-edge collaborator.

## Integration: private surface

The chain `admit_hash_to_scalar_selection` over the real `lookup_hash_to_scalar_declaration` and the member's declaration constant through the family's private path; outer-edge collaborators mocked: none; the observable result is the returned `Result`; the entries proven are `every_declared_member_is_admitted_to_the_selection_that_names_it`, `a_mock_member_is_admitted_to_the_selection_naming_the_selected_mock`, and `a_setting_supplied_for_a_member_is_refused_naming_the_member`.

## Integration: public surface

An outside caller invokes `admit_hash_to_scalar_selection` with `AdmitHashToScalarSelectionDeps` holding `lookup_hash_to_scalar_declaration`, `AdmitHashToScalarSelectionParams { concrete }`, and a `serde_json::Value`, and observes the returned `Result`; the entries the public integration test proves are `every_declared_member_is_admitted_to_the_selection_that_names_it`, `a_mock_member_is_admitted_to_the_selection_naming_the_selected_mock`, and `a_setting_supplied_for_a_member_is_refused_naming_the_member`, carrying both callee entries above.
