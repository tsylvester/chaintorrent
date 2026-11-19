# `admit_generate_composition` — interaction spec

Branch contract for the `admit_generate_composition` module of the `generate` module of `apps/harness-crypto`: `admit_generate_composition` reads the declaration of the member `params.identifier` names through the declaration lookup its deps carry, compares the chain forms and encoding identifiers that declaration requires with the identifiers the received chain forms and encoder declarations carry, and returns success where both agree or the family's `Incompatible` refusal naming each required and declared identifier that disagrees — before anything is constructed, constructing nothing and having no side effect. Each function's branches state condition, decision, dependency call, and the exact return outcome.

## `admit_generate_composition`

`admit_generate_composition(deps: &AdmitGenerateCompositionDeps, params: AdmitGenerateCompositionParams, payload: AdmitGenerateCompositionPayload<'_>) -> AdmitGenerateCompositionReturn`, the trusted form; the function holds no branch on the identifier, the handle selecting the member's declaration.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| declaration read | every call | none | `(deps.lookup_generate_declaration)(&LookupGenerateDeclarationDeps, LookupGenerateDeclarationParams { identifier: params.identifier }, LookupGenerateDeclarationPayload)`, bound through the irrefutable pattern `let Ok(looked_up) = …;` because its error arm is `Infallible` | the member's declaration is `looked_up.declaration` |
| chain forms comparison | the declaration read | `looked_up.declaration.chain_forms_identifier` is not equal to `payload.chain_forms.identifier` | none | `Some(ChainFormsIdentifierDisagreement { required: looked_up.declaration.chain_forms_identifier, declared: payload.chain_forms.identifier })` where unequal and `None` where equal |
| encoding comparison | the declaration read | `looked_up.declaration.encoding_identifier` is not equal to `payload.encoder.identifier` | none | `Some(EncodingIdentifierDisagreement { required: looked_up.declaration.encoding_identifier, declared: payload.encoder.identifier })` where unequal and `None` where equal |
| incompatible | at least one of the two comparison outcomes is `Some` | the two outcomes | none | `Err(AdmitGenerateCompositionErrorReturn::Incompatible { chain_forms, encoding })` holding both outcomes, so every disagreement is named in one refusal |
| compatible | both comparison outcomes are `None` | none | none | `Ok(AdmitGenerateCompositionSuccessReturn)` |

The chain forms comparison's `Some` outcome is proven by no block of `test.rs` or `integration_test.rs`, because no input of the official mocks reaches it: the chain family's mock and real forms declarations carry the identifier every member's declaration requires, and no official mock supplies a forms declaration at another identifier.

### Ordering

The declaration is read first, then both comparisons, then the outcome returns; no side effect.

## Integration: callee dispositions

- `harness-crypto/generate/lookup_generate_declaration` entry `Own entries`, each of its two branches proven by its unit block: absorbed, proven by `the_requirement_is_read_from_the_member_the_identifier_names` in the private integration test element `integration_test.rs`, restated in this node's terms as: the declaration the handle returns for each member identifier is the declaration that member owns, so the comparison reads that member's requirements.
- `harness-crypto/generate/solidity/settings`, `harness-crypto/generate/solidity`, `chain/create_chain_forms`, and `encoding/factory`: no entry of their interaction specs passes through a function of this node or ends at one of its tests, this node calling none of their functions and reading their declaration types alone.

## Integration: own entries

- `every_declared_member_is_admitted_against_the_identifiers_it_requires`: condition `admit_generate_composition` is called over each member of `GENERATE_CONCRETES` through the real declaration lookup, with a chain forms declaration and an encoder declaration carrying the identifiers that member's declaration requires; outcome `Ok(AdmitGenerateCompositionSuccessReturn)`; variation the member.
- `the_requirement_is_read_from_the_member_the_identifier_names`: condition `admit_generate_composition` is called through the real declaration lookup over the identifiers `GenerateConcreteKind::Solidity` and `GenerateConcreteKind::Mock`, each with an encoder declaring the other member's identifier and with an encoder declaring the named member's own; outcome `Err(AdmitGenerateCompositionErrorReturn::Incompatible)` holding the encoding disagreement of the named member's required identifier and the declared one, and no chain forms disagreement, when the encoder declares the other member's identifier, and `Ok(AdmitGenerateCompositionSuccessReturn)` when the encoder declares the named member's own; variation the member the identifier names; edge that must survive the composition: the refusal names the required identifier of the member the identifier names and no concrete.

## Integration: private surface

The real functions `admit_generate_composition` and `lookup_generate_declaration`; the outer-edge collaborators mocked are none, the lookup reading constants alone; the observable result is the `AdmitGenerateCompositionReturn`; the entries proven are the two own entries above and the absorbed lookup entry by the blocks of `integration_test.rs`.

## Integration: public surface

The crate's `lib.rs` re-exports `generate::provides`, which re-exports this module's `provides`, so the interface types and, under the `mocks` feature, the builders and the function mock are public; the function is exported at crate visibility, so no outside caller invokes it and no outcome is observable through the public surface, and this node carries no public integration test.
