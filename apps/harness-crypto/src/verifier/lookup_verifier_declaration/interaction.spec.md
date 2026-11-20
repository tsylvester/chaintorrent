# `lookup_verifier_declaration` — interaction spec

Branch contract for the `lookup_verifier_declaration` module of the `verifier` module of `apps/harness-crypto`: `lookup_verifier_declaration` returns the declaration of the verifier member `params.identifier` names, read from that member's own module, so a level that holds an identifier reads the verifier forms, the delivery-statement versions, the chain forms identifier, the chain member, the encoding identifier, the contract suite, and every value the member fixes, without constructing the member. Each branch states condition, decision, dependency call, and the exact return outcome.

## `lookup_verifier_declaration`

`lookup_verifier_declaration(deps: &LookupVerifierDeclarationDeps, params: LookupVerifierDeclarationParams, payload: LookupVerifierDeclarationPayload) -> LookupVerifierDeclarationReturn`, the trusted form; `deps` and `payload` are not read; the decision is an exhaustive `match` on `params.identifier`, so a further member adds an arm.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| EVM | `params.identifier` is `VerifierConcreteKind::Evm` | none | none | `Ok(LookupVerifierDeclarationSuccessReturn { declaration: EVM_VERIFIER_DECLARATION })` |
| Mock, under `#[cfg(any(test, feature = "mocks"))]` | `params.identifier` is `VerifierConcreteKind::Mock` | none | none | `Ok(LookupVerifierDeclarationSuccessReturn { declaration: MOCK_I_VERIFIER_ADAPTER_DECLARATION })` |

### Ordering

The matched branch returns; no side effect.

## Integration: callee dispositions

`harness-crypto/verifier/evm/settings`: this node's spec declares no own entry and no private surface, so no entry of its interaction spec passes through a function of this node or ends at one of its tests, this node calling none of its functions and reading its constant alone.

`harness-crypto/verifier/evm`: no entry of its interaction spec passes through a function of this node or ends at one of its tests, this node calling none of the adapter's functions and reading the family's declaration constants alone.

## Integration: own entries

Each branch calls no function, so no cross-module property exists, and each is proven by its unit block in `test.rs`.

## Integration: private surface

The real function `lookup_verifier_declaration`, which calls no function; the observable result is the `LookupVerifierDeclarationReturn`; the entries proven are the unit blocks of `test.rs`, and the node carries no private integration test.

## Integration: public surface

The crate's `lib.rs` re-exports `verifier::provides`, which re-exports this module's `provides`, so the interface types and, under the `mocks` feature, the builders and the function mock are public; the function is exported at crate visibility, so no outside caller invokes it and no outcome is observable through the public surface, and this node carries no public integration test.
