# lookup_hashing_declaration interactions

`lookup_hashing_declaration` is the hashing family's declaration lookup: it reads the `CommitmentDeclaration` of the member `params.concrete` names, from that member's interface-held declaration constant, with no adapter constructed. A caller that holds an admitted member identifier learns what the member declares — its commitment-scheme identifier, chunk length, key length, adapter version, and interface version — before any construction, and no caller restates a member's declared value from its own knowledge of the member.

## `lookup_hashing_declaration`

`lookup_hashing_declaration(deps: &LookupHashingDeclarationDeps, params: LookupHashingDeclarationParams, payload: LookupHashingDeclarationPayload) -> LookupHashingDeclarationReturn`

The decision is a `match` on `params.concrete`, one arm per `CommitmentConcrete` variant, exhaustive so a variant with no arm fails to compile and a further concrete of the hashing family adds its own arm.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| blake3_bao | `params.concrete` is `CommitmentConcrete::Blake3Bao` | the exhaustive `match` on `params.concrete` | none; the arm reads the interface-held constant `Blake3BaoCommitment::DECLARATION` and constructs no adapter | `Ok(LookupHashingDeclarationSuccessReturn { declaration })`, `declaration` equal in every field to that constant |
| mock | `params.concrete` is `CommitmentConcrete::Mock(failure_mode)`, under `#[cfg(any(test, feature = "mocks"))]` | the same `match` | none; the arm reads the family's `MOCK_I_COMMITMENT_ADAPTER_DECLARATION` | `Ok(LookupHashingDeclarationSuccessReturn { declaration })`, `declaration` holding every field of `MOCK_I_COMMITMENT_ADAPTER_DECLARATION` except `concrete`, which is `CommitmentConcrete::Mock(failure_mode)` with the `failure_mode` the params selected |

The error arm has no branch, since no input reaches it, and it has no unit test.

Ordering and effects: the function reads constants, constructs nothing, mutates nothing, and retains no params or payload; the same params always yield the same outcome.

## Integration: callee dispositions

None. The direct dependency `hashing/blake3_bao` carries the entries `commit_under_a_key_leaves_the_borrowed_key_intact`, `committed_outboard_verifies_its_content`, `extracted_paths_verify_every_chunk`, `a_challenge_response_verifies_against_the_root`, and `keyed_paths_verify_only_under_their_key`, which concern `commit`, `verify`, `extract_path`, `verify_chunk`, `challenge`, and `respond`; none passes through this function and none ends at one of its tests.

## Integration: own entries

- `every_declared_member_declares_itself_through_the_public_surface`: condition the function is called through the crate's public surface once for each member of `COMMITMENT_CONCRETES`; outcome `Ok` whose declaration's `identifier` is the real `CommitmentSchemeIdentifier::Blake3BaoV1` and whose `interface_version` is `COMMITMENT_INTERFACE_VERSION`; variation the selected member across the declared set; the failure that must survive composition is a declared member whose declaration the surface cannot read.

## Integration: private surface

None. The function composes no collaborator beyond the constant it reads, which the unit tests prove, so the module has no `integration_test.rs`.

## Integration: public surface

An outside caller invokes `lookup_hashing_declaration` from the crate root with params naming each member of `COMMITMENT_CONCRETES` and observes the success return's declaration; the entry proven is `every_declared_member_declares_itself_through_the_public_surface`; its route is the public call, the `match` arm of the selected member, and that member's interface-held declaration constant; no outer-edge collaborator exists. The outcome of a `Mock(failure_mode)` selection, observable under the `mocks` feature, is a property of this function alone and is proven by `lookup_returns_the_mock_declaration_naming_the_selected_failure_mode`, so it is not an entry, and the declared set the public block runs over holds real members alone.
