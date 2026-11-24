# admit_hashing_selection interactions

`admit_hashing_selection` is the hashing family's selection admission: it receives the admitted member identifier `params.concrete` and the untrusted settings document, refuses a document that supplies any setting with the family's own error naming the member, and otherwise returns the selection that names the member. No member of the hashing family has a settable value, so a supplied setting would carry configuration no member reads, and the admission refuses it instead of returning a selection that silently discards it.

The document decision is shared by every member's arm: the document supplies no setting when it is `Value::Null` or `Value::Object` whose map is empty, and supplies a setting when it is any other value.

## `admit_hashing_selection`

`admit_hashing_selection(deps: &AdmitHashingSelectionDeps, params: AdmitHashingSelectionParams, payload: serde_json::Value) -> AdmitHashingSelectionReturn`

The decision is a `match` on `params.concrete`, one arm per `CommitmentConcrete` variant, exhaustive so a variant with no arm fails to compile and a further concrete of the hashing family adds its own arm.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| blake3_bao, document supplies no setting | `params.concrete` is `CommitmentConcrete::Blake3Bao` and the document decision finds no setting | the exhaustive `match` on `params.concrete` and the document decision | none | `Ok(AdmitHashingSelectionSuccessReturn { selection })`, `selection` equal to `CommitmentConcrete::Blake3Bao` |
| blake3_bao, document supplies a setting | `params.concrete` is `CommitmentConcrete::Blake3Bao` and the document decision finds a setting | the same `match` and document decision | none | `Err(AdmitHashingSelectionErrorReturn::SettingsSupplied { concrete: CommitmentConcrete::Blake3Bao })` |
| mock, document supplies no setting | `params.concrete` is `CommitmentConcrete::Mock(failure_mode)`, under `#[cfg(any(test, feature = "mocks"))]`, and the document decision finds no setting | the same `match` and document decision | none | `Ok(AdmitHashingSelectionSuccessReturn { selection })`, `selection` equal to `CommitmentConcrete::Mock(failure_mode)` with the `failure_mode` the params selected |
| mock, document supplies a setting | `params.concrete` is `CommitmentConcrete::Mock(failure_mode)`, under `#[cfg(any(test, feature = "mocks"))]`, and the document decision finds a setting | the same `match` and document decision | none | `Err(AdmitHashingSelectionErrorReturn::SettingsSupplied { concrete })`, `concrete` equal to `CommitmentConcrete::Mock(failure_mode)` with the `failure_mode` the params selected |

Ordering and effects: the function reads its params and payload, constructs nothing, mutates nothing, and retains no params or payload; the same params and payload always yield the same outcome.

## Integration: callee dispositions

None. The direct dependency `hashing/blake3_bao` carries the entries `commit_under_a_key_leaves_the_borrowed_key_intact`, `committed_outboard_verifies_its_content`, `extracted_paths_verify_every_chunk`, `a_challenge_response_verifies_against_the_root`, and `keyed_paths_verify_only_under_their_key`, which concern `commit`, `verify`, `extract_path`, `verify_chunk`, `challenge`, and `respond`; none passes through this function and none ends at one of its tests. The direct dependency `hashing/lookup_hashing_declaration` carries the entry `every_declared_member_declares_itself_through_the_public_surface`, which concerns `lookup_hashing_declaration`; it does not pass through this function and does not end at one of its tests.

## Integration: own entries

- `every_declared_member_is_selected_through_the_public_surface`: condition the function is called through the crate's public surface once for each member of `COMMITMENT_CONCRETES`, first with a document that is `Value::Null` and then with a document that is a JSON object holding an entry; outcome the first call returns `Ok` whose `selection` equals the member the params named, and the second returns `Err(AdmitHashingSelectionErrorReturn::SettingsSupplied { concrete })` with `concrete` equal to that member; variation the selected member across the declared set and the document across absent and supplying a setting; the failure that must survive composition is a supplied setting reaching a selection that discards it, or a refusal that names another member than the one selected.

## Integration: private surface

None. The function composes no collaborator, which the unit tests prove, so the module has no `integration_test.rs`.

## Integration: public surface

An outside caller invokes `admit_hashing_selection` from the crate root with params naming each member of `COMMITMENT_CONCRETES` and a document, and observes the returned selection or the refusal; the entry proven is `every_declared_member_is_selected_through_the_public_surface`; its route is the public call and the `match` arm of the selected member; no outer-edge collaborator exists. The outcome of a `Mock(failure_mode)` selection, observable under the `mocks` feature, is a property of this function alone and is proven by `admit_returns_the_mock_selection_naming_the_selected_failure_mode` and `admit_refuses_a_supplied_setting_naming_the_mock_member`, so it is not an entry; the outcome of an empty-object document and the outcome of a non-object document, each observable through the public call, are properties of this function alone and are proven by `admit_returns_the_blake3_bao_selection_for_an_empty_document` and `admit_refuses_a_document_that_is_not_an_object`, so they are not entries; the declared set the public block runs over holds real members alone.
