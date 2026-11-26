# `admit_kem_composition` — interaction spec

Branch contract for the `admit_kem_composition` module of the `kem` crate: a family-owned function that admits a composition of the member `params.concrete` under the identity scope `params.scope` the suite requires, before the factory constructs the member and before any effort. Each branch states condition, decision, dependency call, and the exact return outcome.

## `admit_kem_composition(deps: &AdmitKemCompositionDeps, params: AdmitKemCompositionParams, payload: AdmitKemCompositionPayload) -> AdmitKemCompositionReturn`

The declaration is read once, before any other step, by calling `deps.lookup_kem_declaration`; the same two branches decide for the real and the mock member, the member's declaration being the only input that differs. The lookup's `Err` arm is `Infallible`, uninhabited, so no branch handles or returns it.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| scope provided | the declaration `deps.lookup_kem_declaration` returns for `params.concrete` has `identity_scopes` containing `params.scope` | the declaration is read once, before any other step, by calling `deps.lookup_kem_declaration` with `LookupKemDeclarationDeps`, `LookupKemDeclarationParams { concrete: params.concrete }`, and `LookupKemDeclarationPayload`, and `contains` over `identity_scopes` decides | `deps.lookup_kem_declaration` once | `Ok(AdmitKemCompositionSuccessReturn { concrete: params.concrete, scope: params.scope })` |
| scope not provided | the declaration's `identity_scopes` does not contain `params.scope` | the same read and the same `contains` | `deps.lookup_kem_declaration` once | `Err(AdmitKemCompositionErrorReturn::Incompatible { concrete: params.concrete, declared: declaration.identity_scopes, required: params.scope })`, with nothing constructed |

## Ordering and edges

- Side effects none; no member is constructed, no setting is read, and no adapter is received. The decision is computed from the declaration and `params.scope` alone.
- `payload` carries nothing and is not read.

## Integration: callee dispositions

`kem/lookup_kem_declaration`'s entry `every_member_of_the_declared_set_has_a_declaration_that_names_it` passes through `admit_kem_composition`, which calls the lookup, and is carried; `kem/admit_kem_selection`'s entries `every_member_of_the_declared_set_is_admitted_to_a_selection_the_declaration_lookup_reads` and `a_document_that_supplies_a_setting_is_refused_naming_each_member_of_the_declared_set` end at `admit_kem_selection`, which this function does not call, and pass through none of its functions or tests.

## Integration: own entries

Each entry is proven by the block of the same name in this node's public integration test element `tests/admit_kem_composition_integration_test.rs`.

- `a_member_admitted_by_selection_is_compatible_with_each_scope_of_the_declared_set`: condition `admit_kem_selection` called through the crate's public surface with each member of `KEM_CONCRETES` and `Value::Null`, then `admit_kem_composition` called with the selection it returned as `params.concrete`, each member of `IDENTITY_SCOPES` as `params.scope`, and deps holding the real `lookup_kem_declaration`; outcome `Ok(AdmitKemCompositionSuccessReturn { concrete: member, scope })` for each pair; variation each member of `KEM_CONCRETES` against each member of `IDENTITY_SCOPES`; edge a composition that refused an admitted member under a scope the family declares, or that named another member or another scope in its success, fails the comparison.

Carried entry `every_member_of_the_declared_set_has_a_declaration_that_names_it`: restated in this node's terms, the declaration this function compares for each member of `KEM_CONCRETES` is that member's own, so each member is compatible with every scope of `IDENTITY_SCOPES`, observed as `Ok(AdmitKemCompositionSuccessReturn { concrete: member, scope })` for each pair and never as an `Incompatible` naming another member; route `lookup_kem_declaration`, which reads the constant the concrete declares, then `admit_kem_composition`, with no outer-edge collaborator; proven by the block of the public integration test element of the same name.

## Integration: private surface

This spec lists no private-surface entry: the real chain `admit_kem_composition` then `lookup_kem_declaration` is observable through the crate's public surface and proven there, and the `Incompatible` arm is reached only by a declaration the real members do not declare, so no input to the real chain reaches it and it is proven by the unit blocks.

## Integration: public surface

An outside caller invokes `admit_kem_composition` with deps from `build_admit_kem_composition_deps` overriding `lookup_kem_declaration` with the real `lookup_kem_declaration`, params from `build_admit_kem_composition_params`, and a payload from `build_admit_kem_composition_payload`, and `admit_kem_selection` with the builders of that module, reached through the crate root's re-exports of those modules' `provides` and the `mocks` feature; it observes `Ok(success)` and its `concrete` and `scope`; the entries proven are `a_member_admitted_by_selection_is_compatible_with_each_scope_of_the_declared_set` and the carried `every_member_of_the_declared_set_has_a_declaration_that_names_it`; the route of the first is the public call `admit_kem_selection`, then the public call `admit_kem_composition`, which calls `lookup_kem_declaration`, with no outer-edge collaborator; the route of the carried entry is the public call `admit_kem_composition` and the `lookup_kem_declaration` it calls; the mock member's outcomes are proven by the unit blocks, the declared set `KEM_CONCRETES` excluding it.
