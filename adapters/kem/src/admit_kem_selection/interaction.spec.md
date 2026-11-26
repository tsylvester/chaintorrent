# `admit_kem_selection` — interaction spec

Branch contract for the `admit_kem_selection` module of the `kem` crate: the family's selection admission, turning the `KemConcrete` member identifier admitted from configuration and the untrusted settings document into the selection that names the member, before any member is constructed. Each branch states condition, decision, dependency call, and the exact return outcome.

## `admit_kem_selection(deps: &AdmitKemSelectionDeps, params: AdmitKemSelectionParams, payload: serde_json::Value) -> AdmitKemSelectionReturn`

The document decision, stated once for every branch below: the document supplies a setting unless `payload` is `Value::Null` or a `Value::Object` whose entries are empty; the decision is computed from the payload alone, before any branch, and calls no dependency.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| real member admitted | `params.concrete` is `KemConcrete::Bb1DepthOne` and the document supplies no setting | an exhaustive `match` on `params.concrete` after the document decision | none | `Ok(AdmitKemSelectionSuccessReturn { selection: KemConcrete::Bb1DepthOne })` |
| real member refused | `params.concrete` is `KemConcrete::Bb1DepthOne` and the document supplies a setting | the same | none | `Err(AdmitKemSelectionErrorReturn::SettingsSupplied { concrete: KemConcrete::Bb1DepthOne })` |
| mock member admitted — present only under `#[cfg(any(test, feature = "mocks"))]` | `params.concrete` is `KemConcrete::Mock` and the document supplies no setting | the same `match` | none | `Ok(AdmitKemSelectionSuccessReturn { selection: KemConcrete::Mock })` |
| mock member refused — present only under `#[cfg(any(test, feature = "mocks"))]` | `params.concrete` is `KemConcrete::Mock` and the document supplies a setting | the same `match` | none | `Err(AdmitKemSelectionErrorReturn::SettingsSupplied { concrete: KemConcrete::Mock })` |

## Ordering and edges

- Side effects none; no member is constructed, no declaration is read, and no adapter is received.
- The `match` is exhaustive over `KemConcrete`, so a variant without an arm does not compile.
- `deps` carries no value and is not read.
- A further concrete adds its arm to the `match` in its own increment.

## Integration: own entries

Each entry is proven by the block of the same name in this node's public integration test element `tests/admit_kem_selection_integration_test.rs`.

- `every_member_of_the_declared_set_is_admitted_to_a_selection_the_declaration_lookup_reads`: condition `admit_kem_selection` called through the crate's public surface with each member of `KEM_CONCRETES` and a document that supplies no setting, then `lookup_kem_declaration` called with the selection the admission returned; outcome the admission is `Ok` holding a selection equal to the member and the lookup is `Ok` holding a declaration whose `concrete` equals the member; variation each member of `KEM_CONCRETES` and each document that supplies no setting, `Value::Null` and a `Value::Object` with no entries; edge an admission that returned another member's selection, or a selection the declaration lookup does not resolve to the same member, fails the comparison.
- `a_document_that_supplies_a_setting_is_refused_naming_each_member_of_the_declared_set`: condition `admit_kem_selection` called through the crate's public surface with each member of `KEM_CONCRETES` and a document that supplies a setting; outcome `Err(AdmitKemSelectionErrorReturn::SettingsSupplied { concrete })` with `concrete` equal to the member; variation each member of `KEM_CONCRETES` and each document that supplies a setting, a `Value::Object` holding an entry, an empty `Value::Array`, a `Value::Bool`, a `Value::Number`, and a `Value::String`, against the documents that supply none, which the entry above shows admitted; edge an admission that returned `Ok` for any of these documents, or an error naming another member, fails the comparison.

## Integration: callee dispositions

Every entry of `kem/bb1_depth_one`'s interaction spec ends at a method of `Bb1DepthOneKem` or at `Bb1DepthOneKem::try_new`, and every entry of `kem/lookup_kem_declaration`'s interaction spec ends at `lookup_kem_declaration`; none passes through `admit_kem_selection` or ends at one of its tests, so no entry is absorbed or carried.

## Integration: private surface

This spec lists no private-surface entry: the function reads no constant and calls no function, so no chain of real functions runs inside the crate.

## Integration: public surface

An outside caller invokes `admit_kem_selection` with deps and params from `build_admit_kem_selection_deps` and `build_admit_kem_selection_params` and a payload written through `Value`'s own variants, and `lookup_kem_declaration` with the builders of that module, reached through the crate root's re-exports of both modules' `provides` and the `mocks` feature; it observes the `Ok` selection or the `Err` variant and, for the lookup, `success.declaration.concrete`; the entries proven are `every_member_of_the_declared_set_is_admitted_to_a_selection_the_declaration_lookup_reads` and `a_document_that_supplies_a_setting_is_refused_naming_each_member_of_the_declared_set`; the route is the public call `admit_kem_selection`, extended in the first entry by the public call `lookup_kem_declaration`, with no outer-edge collaborator; the mock member's outcomes are proven by the unit blocks, the declared set `KEM_CONCRETES` excluding it.
