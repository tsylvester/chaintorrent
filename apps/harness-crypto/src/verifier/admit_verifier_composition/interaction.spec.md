# `admit_verifier_composition` — interaction spec

Branch contract for the `admit_verifier_composition` module of the `verifier` module of `apps/harness-crypto`: `admit_verifier_composition` reads the declaration of the member `params.identifier` names through the declaration lookup its deps carry, compares the chain forms identifier, the chain member, the encoding identifier, the verifier forms, and the contract suite that declaration requires with the values the received chain forms, chain, encoder, pairing, and generate declarations carry, and returns success where all five agree or the family's `Incompatible` refusal naming each required and declared value that disagrees — before anything is constructed, constructing nothing and having no side effect. Each function's branches state condition, decision, dependency call, and the exact return outcome.

## `admit_verifier_composition`

`admit_verifier_composition(deps: &AdmitVerifierCompositionDeps, params: AdmitVerifierCompositionParams, payload: AdmitVerifierCompositionPayload<'_>) -> AdmitVerifierCompositionReturn`, the trusted form; the function holds no branch on the identifier, the handle selecting the member's declaration.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| declaration read | every call | none | `(deps.lookup_verifier_declaration)(&LookupVerifierDeclarationDeps, LookupVerifierDeclarationParams { identifier: params.identifier }, LookupVerifierDeclarationPayload)`, bound through the irrefutable pattern `let Ok(looked_up) = …;` because its error arm is `Infallible` | the member's declaration is `looked_up.declaration` |
| chain forms comparison | the declaration read | `looked_up.declaration.chain_forms_identifier` is not equal to `payload.chain_forms.identifier` | none | `Some(ChainFormsIdentifierDisagreement { required: looked_up.declaration.chain_forms_identifier, declared: payload.chain_forms.identifier })` where unequal and `None` where equal |
| chain member comparison | the declaration read | `looked_up.declaration.chain_member` is not equal to `payload.chain.identifier` | none | `Some(ChainMemberIdentifierDisagreement { required: looked_up.declaration.chain_member, declared: payload.chain.identifier })` where unequal and `None` where equal |
| encoding comparison | the declaration read | `looked_up.declaration.encoding_identifier` is not equal to `payload.encoder.identifier` | none | `Some(EncodingIdentifierDisagreement { required: looked_up.declaration.encoding_identifier, declared: payload.encoder.identifier })` where unequal and `None` where equal |
| verifier form comparison | the declaration read | `looked_up.declaration.verifier_forms` does not contain `payload.pairing.verifier_group_arithmetic` | none | `Some(VerifierFormDisagreement { required: looked_up.declaration.verifier_forms, declared: payload.pairing.verifier_group_arithmetic })` where the form is not served and `None` where it is served |
| contract suite comparison | the declaration read | `looked_up.declaration.contract_suite` is not equal to `payload.generate.contract_suite` | none | `Some(ContractSuiteDisagreement { required: looked_up.declaration.contract_suite, declared: payload.generate.contract_suite })` where unequal and `None` where equal |
| incompatible | at least one of the five comparison outcomes is `Some` | the five outcomes | none | `Err(AdmitVerifierCompositionErrorReturn::Incompatible { chain_forms, chain_member, encoding, verifier_form, contract_suite })` holding all five outcomes, so every unmet requirement is named in one refusal |
| compatible | all five comparison outcomes are `None` | none | none | `Ok(AdmitVerifierCompositionSuccessReturn)` |

The chain forms comparison's `Some` outcome is proven by no block of `test.rs` or `integration_test.rs`, because no input of the official mocks reaches it: the chain family's mock and real forms declarations carry the identifier every member's declaration requires, and no official mock supplies a forms declaration at another identifier.

### Ordering

The declaration is read first, then the five comparisons, then the outcome returns; no side effect.

## Integration: callee dispositions

- `harness-crypto/verifier/lookup_verifier_declaration`: the node's spec declares each branch proven by its unit block and carries no private integration test, so no entry of its spec passes through a function of this node or ends at one of its tests; the property that the lookup returns a member's own declaration is proven by the lookup's own unit blocks, and the first own entry below reads each member's requirements through the real lookup.
- `harness-crypto/verifier/evm/settings` and `harness-crypto/verifier/evm`: no entry of either spec passes through a function of this node or ends at one of its tests, this node calling none of their functions and reading the family's declaration constants alone, through the lookup.
- This node calls no function of `chain/create_chain_forms`, `chain/lookup_chain_declaration`, `chain/admit_chain_composition`, `encoding/factory`, `pairing/factory`, `harness-crypto/generate/factory`, or `harness-crypto/generate/solidity/settings` and reads their declaration types alone, so no entry of their specs passes through a function of this node or ends at one of its tests.

## Integration: own entries

- `every_declared_member_is_admitted_against_the_values_it_requires`: over each member of `VERIFIER_CONCRETES` and, within each, over each verifier form `FirstGroupOnly` and `BothGroups`, the composition admission read through the real declaration lookup returns success for a chain forms declaration, a chain declaration, an encoder declaration, and a generate declaration carrying the values that member's declaration requires and a pairing declaration carrying that form; the variation is the member and the form the pairing declares, so a branch that served one form only, or that read another member's requirements, fails.
- `the_requirement_is_read_from_the_member_the_identifier_names`: over the identifiers `VerifierConcreteKind::Evm` and `VerifierConcreteKind::Mock`, the composition admission read through the real declaration lookup returns `Incompatible` holding the chain member disagreement of the named member's required chain member and the declared one, and no other disagreement, when the chain declaration is the other member's, and returns success when the chain declaration is the named member's own; the variation is the member the identifier names, and the failure that must survive is the refusal naming the required chain member of the member the identifier names and no concrete.

## Integration: private surface

The real functions `admit_verifier_composition` and `lookup_verifier_declaration`, the lookup reached through `ADMIT_VERIFIER_COMPOSITION_DEPS`; the outer-edge collaborators mocked are none, the lookup reading constants alone; the observable result is the `AdmitVerifierCompositionReturn`; the entries proven are the two own entries above, each by its block of `integration_test.rs`.

## Integration: public surface

The crate's `lib.rs` re-exports `verifier/provides.rs`, which re-exports this module's `provides.rs`, so the interface types and, under the `mocks` feature, the builders and the function mock are public; the function is exported at crate visibility, so no outside caller invokes it and no outcome is observable through the public surface, and this node carries no public integration test.
