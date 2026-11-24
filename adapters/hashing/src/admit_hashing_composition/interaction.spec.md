# admit_hashing_composition interactions

`admit_hashing_composition` is the hashing family's composition admission: it compares the commitment-scheme identifier the selected member declares, read through `deps.lookup_hashing_declaration` with no adapter constructed, against the commitment-scheme identifier `params.identifier` requires, so a member that does not provide the required identifier is refused before construction and before effort, and no caller restates a member's declared identifier from its own knowledge of the member.

## `admit_hashing_composition`

`admit_hashing_composition(deps: &AdmitHashingCompositionDeps, params: AdmitHashingCompositionParams, payload: AdmitHashingCompositionPayload) -> AdmitHashingCompositionReturn`

The declaration read: the function calls `deps.lookup_hashing_declaration` with a `LookupHashingDeclarationDeps`, `LookupHashingDeclarationParams { concrete: params.concrete }`, and a `LookupHashingDeclarationPayload`, built by struct literals; the lookup's error arm has no variant, so the call binds through the irrefutable pattern `let Ok(looked_up) = …;`, and the branches below read `looked_up.declaration.identifier`.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| identifiers agree | the declaration's `identifier` equals `params.identifier` | equality of two `CommitmentSchemeIdentifier` values | the lookup above and no other | `Ok(AdmitHashingCompositionSuccessReturn)` |
| identifiers disagree | the declaration's `identifier` differs from `params.identifier` | the same equality | the same lookup | `Err(AdmitHashingCompositionErrorReturn::Incompatible { concrete: params.concrete, declared: the declaration's identifier, required: params.identifier })` |

The disagreeing branch has no unit test: no input and no failure mode of the official mocks reaches it, since `mock_lookup_hashing_declaration` and the real lookup return, for every member and every `MockICommitmentAdapterFailureMode`, a declaration holding the identifier the real member provides, and a value identifier has a variant only for a value some member of the declared set provides; for the same reason the agreeing-branch blocks vary no input the comparison discriminates over.

The function is the same for every member of `CommitmentConcrete` present in the build, since it reads the member's declaration through the lookup and matches on no member.

Ordering and effects: the function constructs nothing, mutates nothing, and retains no params or payload; the same deps, params, and payload always yield the same outcome.

## Integration: callee dispositions

Carried `every_declared_member_declares_itself_through_the_public_surface` of `hashing/lookup_hashing_declaration`, restated where this node transforms it: the declaration the lookup returns for each member of `COMMITMENT_CONCRETES` is read by this function and compared with the identifier required, so every declared member is admitted for the identifier it declares; its route is that member's interface-held declaration constant, the `match` arm of the selected member, the injected `lookup_hashing_declaration`, and `admit_hashing_composition`; no outer-edge collaborator exists; no other direct dependency entry passes through this function or ends at one of its tests.

## Integration: own entries

None. The cross-module property of reading each member's real declaration is the carried entry above, stated in this node's terms.

## Integration: private surface

None. The function composes no collaborator beyond the lookup its deps carry, which the public integration test runs for real, so the module has no `integration_test.rs`.

## Integration: public surface

An outside caller invokes `admit_hashing_composition` from the crate root with deps holding `lookup_hashing_declaration`, params naming each member of `COMMITMENT_CONCRETES` and the identifier the real lookup returns for that member, and observes the returned outcome; the entry proven is the carried `every_declared_member_declares_itself_through_the_public_surface`; its route is that member's interface-held declaration constant, the `match` arm of the selected member, the injected lookup, and the public call. The outcome of a `Mock(failure_mode)` selection, observable under the `mocks` feature, is a property of this function alone and is proven by `admit_returns_ok_for_the_mock_member_naming_the_required_identifier`, so it is not an entry, and the declared set the public block runs over holds real members alone.
