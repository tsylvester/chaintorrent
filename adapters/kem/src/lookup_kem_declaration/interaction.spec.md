# `lookup_kem_declaration` — interaction spec

Branch contract for the `lookup_kem_declaration` module of the `kem` crate: a family-owned function that reads the declaration the member identifier `params.concrete` names, before any instance exists. Each branch states condition, decision, dependency call, and the exact return outcome.

## `lookup_kem_declaration(deps: &LookupKemDeclarationDeps, params: LookupKemDeclarationParams, payload: LookupKemDeclarationPayload) -> LookupKemDeclarationReturn`

The decision is an exhaustive `match` on `params.concrete`, one arm per `KemConcrete` variant — `Bb1DepthOne` and, under `#[cfg(any(test, feature = "mocks"))]`, `Mock` — so a variant with no arm fails to compile. No branch refuses: the error arm of the return is uninhabited.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| real member | `params.concrete` is `KemConcrete::Bb1DepthOne` | an exhaustive `match` on `params.concrete` | none; `BB1_DEPTH_ONE_DECLARATION` is read as a constant | `Ok(LookupKemDeclarationSuccessReturn { declaration: BB1_DEPTH_ONE_DECLARATION })` |
| mock member, under `#[cfg(any(test, feature = "mocks"))]` | `params.concrete` is `KemConcrete::Mock` | the same `match` | none | `Ok(LookupKemDeclarationSuccessReturn { declaration: MOCK_I_CREDENTIAL_KEM_ADAPTER_DECLARATION })`, returned unchanged because `KemConcrete::Mock` carries no field to substitute |

## Ordering and edges

- Side effects none; no member is constructed and no adapter is received. The `match` is exhaustive over `KemConcrete`.
- `deps` holds no collaborator and `payload` carries nothing; neither is read.

## Integration: callee dispositions

Every entry of `kem/bb1_depth_one`'s interaction spec ends at a method of `Bb1DepthOneKem` or at `Bb1DepthOneKem::try_new`; none passes through `lookup_kem_declaration` or ends at one of its tests, so no entry is absorbed or carried here.

## Integration: own entries

Each entry is proven by the block of the same name in this node's public integration test element `tests/lookup_kem_declaration_integration_test.rs`.

- `every_member_of_the_declared_set_has_a_declaration_that_names_it`: condition `lookup_kem_declaration` called through the crate's public surface with each member of `KEM_CONCRETES` and a deps and payload that hold no adapter; outcome `Ok` holding a declaration whose `concrete` equals the member, whose `identifier` is a member of `KEM_IDENTIFIERS`, and whose `identity_scopes` is non-empty with each scope a member of `IDENTITY_SCOPES`; variation each member of `KEM_CONCRETES`; edge a branch that returned another member's declaration, or a declaration naming an identifier or scope outside the family's declared sets, fails the comparison.

## Integration: private surface

This spec lists no private-surface entry: `lookup_kem_declaration` reads a constant and calls no function, so no chain of real functions runs inside the crate.

## Integration: public surface

An outside caller invokes `lookup_kem_declaration` with deps, params, and payload from `build_lookup_kem_declaration_deps`, `build_lookup_kem_declaration_params`, and `build_lookup_kem_declaration_payload`, reached through the crate root's re-export of this module's `provides` and the `mocks` feature; it observes `Ok(success)` and the fields of `success.declaration`; the entry proven is `every_member_of_the_declared_set_has_a_declaration_that_names_it`; the route is the public call, `lookup_kem_declaration`, and the constant it reads, with no outer-edge collaborator.
