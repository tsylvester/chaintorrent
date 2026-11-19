# `admit_chain_composition` — interaction spec

Branch contract for the `admit_chain_composition` module of the `chain` crate: `admit_chain_composition` decides whether the chain member its params name may be composed with the chain forms concrete the declaration in its payload resolved, reading the member's declaration through the declaration lookup its deps carry and comparing the forms identifier that declaration lists with the identifier the received forms declaration declares — before anything is constructed and with no side effect. Each branch states condition, decision, dependency call, and the exact return outcome.

## `admit_chain_composition(deps: &AdmitChainCompositionDeps, params: AdmitChainCompositionParams, payload: AdmitChainCompositionPayload<'_>) -> AdmitChainCompositionReturn`

The trusted form: the params and the payload carry no untrusted data. The function holds no branch on `params.identifier` — the handle selects the member's declaration, so a further member adds no arm here.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| declaration read | every call | none | `(deps.lookup_chain_declaration)(&LookupChainDeclarationDeps, LookupChainDeclarationParams { identifier: params.identifier }, LookupChainDeclarationPayload)`, bound through the irrefutable pattern `let Ok(looked_up) = …;` because its error arm is `Infallible` | the member's declaration is `looked_up.declaration` |
| forms comparison | the declaration read | `looked_up.declaration.forms_identifier` is not equal to `payload.forms.identifier` | none | `Some(ChainFormsIdentifierDisagreement { required: looked_up.declaration.forms_identifier, declared: payload.forms.identifier })` where unequal and `None` where equal |
| incompatible | the comparison outcome is `Some` | the outcome | none | `Err(AdmitChainCompositionErrorReturn::Incompatible { forms })` holding the disagreement, the required and the declared identifier named in one refusal |
| compatible | the comparison outcome is `None` | none | none | `Ok(AdmitChainCompositionSuccessReturn)` |

The incompatible arm is a branch of the contract no supplied input reaches: every declaration the official builders and the real lookup supply lists a forms identifier a caller can match, so no block of `test.rs` or `integration_test.rs` proves the `Some` outcome or this refusal.

### Ordering

The declaration is read first, then the comparison, then the outcome returns; no side effect — no clock, no environment variable, no network, no filesystem.

## Integration: callee dispositions

- `chain/lookup_chain_declaration` entries `each_declared_member_is_looked_up_to_the_declaration_it_owns` and `the_mock_member_is_looked_up_to_the_declaration_the_mock_concrete_owns`: neither passes through a function of this node nor ends at one of its tests, because this node reads `forms_identifier` from the declaration the handle returns and its outcome is a function of that identifier and the received forms identifier alone, so the property that the lookup returns a member's own declaration is proven by the lookup's own tests.
- `chain/base`, `chain/base/settings`, `chain/evm_forms`, and `chain/create_chain_forms`: this node calls no function of these modules and reads their declaration types and constants alone, through the lookup, so no entry of their specs passes through a function of this node or ends at one of its tests.

## Integration: own entries

- `every_declared_member_is_admitted_against_the_forms_identifier_it_lists`: condition `admit_chain_composition` is called once for each identifier of `DECLARED_CHAIN_MEMBERS` through the real declaration lookup, with a forms declaration carrying the identifier that member's declaration lists; outcome `Ok(AdmitChainCompositionSuccessReturn)`; variation each member of the declared set, so an admission that reads a member's requirement anywhere but the lookup fails; edge that must survive the composition: the required identifier reaches the comparison through the lookup alone, no caller restating it.
- `the_mock_member_is_admitted_against_the_forms_identifier_it_lists`: condition the call over `ChainMemberIdentifier::Mock` through the real declaration lookup, with a forms declaration carrying the identifier the mock member's declaration lists; outcome `Ok(AdmitChainCompositionSuccessReturn)`; variation the member the identifier names, the mock member standing outside the declared set, so a lookup or an admission that served the declared members alone fails; edge that must survive the composition: the mock identifier reaches its declaration through the same lookup the declared members use.

## Integration: private surface

The real functions `admit_chain_composition` and `lookup_chain_declaration`, the lookup reached through `ADMIT_CHAIN_COMPOSITION_DEPS`; the outer-edge collaborators mocked are none, the lookup reading constants alone; the observable result is the `AdmitChainCompositionReturn`. The entries proven are `every_declared_member_is_admitted_against_the_forms_identifier_it_lists` and `the_mock_member_is_admitted_against_the_forms_identifier_it_lists`, each by its block of `integration_test.rs`.

## Integration: public surface

The crate's `lib.rs` re-exports `admit_chain_composition::provides`, so the interface types and, under the `mocks` feature, the builders and the function mock are public; the function is exported at crate visibility, so no outside caller invokes it and no outcome is observable through the public surface, and this node carries no public integration test.
