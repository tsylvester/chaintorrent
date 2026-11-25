# `admit_encoding_selection` — interaction spec

Branch contract for the `admit_encoding_selection` module of the `encoding` crate: `admit_encoding_selection`, the encoding family's selection admission, turning the admitted member identifier and the untrusted settings document into the family's selection, refusing a document that supplies any setting because no encoding member has a settable value. Each branch states condition, decision, dependency call, and the exact return outcome.

## `admit_encoding_selection(deps: &AdmitEncodingSelectionDeps, params: AdmitEncodingSelectionParams, payload: serde_json::Value) -> AdmitEncodingSelectionReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| entry | any call | the function computes once whether the document supplies a setting, true when `payload` is neither `Value::Null` nor an `Value::Object` that is empty; `payload.is_null()` and `payload.as_object().is_some_and(|document| document.is_empty())` | none | the binding `supplies_setting`, which every arm decides from |
| `Abi` member, a setting supplied | `params.concrete` is `EncodingConcrete::Abi` and the document supplies a setting | an exhaustive `match` on `params.concrete` with no wildcard arm, so a member with no arm fails to compile | none | `Err(AdmitEncodingSelectionErrorReturn::SettingSupplied { concrete: EncodingConcrete::Abi })` |
| `Abi` member, admitted | `params.concrete` is `EncodingConcrete::Abi` and the document supplies no setting | the same `match` | none | `Ok(AdmitEncodingSelectionSuccessReturn { selection: EncodingConcrete::Abi })` |
| `Mock` member, a setting supplied | `params.concrete` is `EncodingConcrete::Mock`, an arm compiled under `#[cfg(any(test, feature = "mocks"))]` alone, and the document supplies a setting | the same `match` | none | `Err(AdmitEncodingSelectionErrorReturn::SettingSupplied { concrete: EncodingConcrete::Mock })` |
| `Mock` member, admitted | `params.concrete` is `EncodingConcrete::Mock` and the document supplies no setting | the same `match` | none | `Ok(AdmitEncodingSelectionSuccessReturn { selection: EncodingConcrete::Mock })` |

## Ordering

The function reads `payload` once, to decide whether it supplies a setting, then reads `params.concrete`; `deps` holds no field and is not read; no adapter is constructed, no file is touched, and no side effect occurs.

## Integration: own entries and callee dispositions

Callee dispositions: none; the function's callees are `serde_json`'s `Value` and `Map` accessors and the standard library, and the specs of `encoding/abi` and `encoding/lookup_encoding_declaration` declare no entry that passes through this function or ends at its tests; the lookup's own entry `declared_member_declaration_through_the_public_surface` concerns a declaration read this function does not make.

Own entry `declared_member_selection_through_the_public_surface`: an outside caller that holds a member of `ENCODING_CONCRETES` and calls `admit_encoding_selection` through the crate's public surface with a document that supplies no setting receives the selection naming that member; the input variation that separates a pass from a fail is each member of the declared set, so an admission that returns another member, or one for a member the barrel does not export, fails; the outcome that must survive composition is the selection naming the member passed, whole.

Own entry `supplied_setting_refused_through_the_public_surface`: an outside caller that holds a member of `ENCODING_CONCRETES` and calls `admit_encoding_selection` through the crate's public surface with a document that supplies a setting receives the family's `SettingSupplied` error naming that member, whole; the input variation that separates a pass from a fail is each member of the declared set paired with a non-empty document, so an admission that admits the document, or names another member in the error, fails; the failure that must survive composition is the error naming the member passed, whole.

Private surface: none; every branch is proven by this node's unit tests.

Public surface: the outside caller invokes `admit_encoding_selection` from the crate root with builder-made deps and params, where the params' `concrete` is each member of `ENCODING_CONCRETES`, and a `serde_json::Value` document, and observes the returned selection or error; the entries the public integration test proves are `declared_member_selection_through_the_public_surface` and `supplied_setting_refused_through_the_public_surface`; the route is the crate barrel, then `admit_encoding_selection`, ending at the returned selection or error; the outer-edge collaborators are none.
