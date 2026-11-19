# `lookup_generate_declaration` — interaction spec

Branch contract for the `lookup_generate_declaration` module of the `generate` module of `apps/harness-crypto`: `lookup_generate_declaration` returns the declaration of the generate member `params.identifier` names, read from that member's own module, so a level holding an identifier reads what the member declares without constructing the member. Each branch states condition, decision, dependency call, and the exact return outcome.

## `lookup_generate_declaration`

`lookup_generate_declaration(deps: &LookupGenerateDeclarationDeps, params: LookupGenerateDeclarationParams, payload: LookupGenerateDeclarationPayload) -> LookupGenerateDeclarationReturn`, the trusted form; `deps` and `payload` are not read; the decision is an exhaustive `match` on `params.identifier`, so a further member adds an arm.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| Solidity | `params.identifier` is `GenerateConcreteKind::Solidity` | none | none | `Ok(LookupGenerateDeclarationSuccessReturn { declaration: SOLIDITY_GENERATE_DECLARATION })` |
| Mock, under `#[cfg(any(test, feature = "mocks"))]` | `params.identifier` is `GenerateConcreteKind::Mock` | none | none | `Ok(LookupGenerateDeclarationSuccessReturn { declaration: MOCK_I_GENERATE_ADAPTER_DECLARATION })` |

### Ordering

The matched branch returns; no side effect.

## Integration: callee dispositions

No entry of the interaction specs of `harness-crypto/generate/solidity/settings` or `harness-crypto/generate/solidity` passes through a function of this node or ends at one of its tests, this node calling none of their functions and reading their constants alone.

## Integration: own entries

None; the function calls no function, so no cross-module property exists, and each branch is proven by its unit block in `test.rs`.

## Integration: private surface

None; this node carries no private integration test.

## Integration: public surface

The crate's `lib.rs` re-exports `generate::provides`, which re-exports this module's `provides`, so the interface types and, under the `mocks` feature, the builders and the function mock are public; the function is exported at crate visibility, so no outside caller invokes it and no outcome is observable through the public surface, and this node carries no public integration test.
