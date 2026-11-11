# factory interactions

`create_commitment` is the hashing family's construction point: it selects the concrete `params.concrete` names, admits it only when its `DECLARATION.identifier` is the `params.identifier` the hash-card requires, and returns it behind `Box<dyn ICommitmentAdapter>` with its declaration. It reads no hash-card and no configuration — the composition resolver passes both as typed values — and it does not hash; every commitment operation is the concrete's.

## `create_commitment`

`create_commitment(deps: &CreateCommitmentDeps, params: CreateCommitmentParams, payload: CreateCommitmentPayload) -> CreateCommitmentReturn`

The decision is a `match` on `params.concrete`, one arm per `CommitmentConcrete` variant, exhaustive so a variant with no arm fails to compile. Within each arm:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| unsupported identifier | the named concrete's `DECLARATION.identifier` is not `params.identifier` | equality, read before any construction | none | `Err(CreateCommitmentErrorReturn::UnsupportedCommitmentScheme)`; nothing is constructed. Under the `mocks` feature the mock concrete declares `CommitmentSchemeIdentifier::MockV1`, so a `Mock` selection paired with another concrete's identifier takes this branch |
| admitted | the named concrete's declared identifier is `params.identifier` | the equality above | the concrete's `try_new` with its fieldless constructor params, exactly once, its success destructured irrefutably because its error arm is uninhabited | `Ok(CreateCommitmentSuccessReturn { adapter: Box::new(commitment) })`, whose adapter reports the concrete's `DECLARATION` |

`Mock` arm, under `#[cfg(any(test, feature = "mocks"))]`: the same admission over `MockICommitmentAdapter::DECLARATION.identifier`; dependency call the mock concrete's `try_new` with `MockICommitmentAdapterConstructorParams { failure_mode }`, the failure mode the selection carries, exactly once, its success destructured irrefutably; the outcome is `Ok(CreateCommitmentSuccessReturn { adapter: Box::new(mock) })`, whose adapter reports `CommitmentSchemeIdentifier::MockV1` and answers under the failure mode.

`CreateCommitmentErrorReturn::Blake3Bao` and `CreateCommitmentErrorReturn::Mock` carry the constructors' uninhabited error type `Infallible` in the return union, so no branch produces them.

`params.concrete` selects and `params.identifier` admits; `deps` and `payload` carry nothing and are not read.

## Shared terms

- `L` is the returned adapter's `declaration().chunk_length`.
- The reference content is `reference_content(REFERENCE_CONTENT_LENGTH)`.
- Its chunk count is `REFERENCE_CONTENT_LENGTH` as a `u64` divided by `L`, rounded up.
- Chunk `i` is the content's bytes from `i × L` to the lesser of `(i + 1) × L` and the content's length.

## Integration: callee dispositions

The entries of `hashing/blake3_bao` that this node's chains reach, each dispositioned as stated.

- `commit_under_a_key_leaves_the_borrowed_key_intact` (`hashing/blake3_bao`): absorbed; proven by `every_concrete_leaves_the_borrowed_key_intact` in this module's private integration test `integration_test.rs`, which runs the entry through `create_commitment` over the declared set and the mock selection.
- `committed_outboard_verifies_its_content` (`hashing/blake3_bao`): absorbed; proven by `every_concrete_verifies_the_content_it_committed` in `integration_test.rs`, restated in this node's terms: the changed byte is at offset `2 × L`, the first byte of chunk 2 under each concrete's declared chunk length, and the outcome for the changed content is `Err(VerifyErrorReturn::ChunkMismatch { chunk_index: 2 })`.
- `extracted_paths_verify_every_chunk` (`hashing/blake3_bao`): absorbed; proven by `every_concrete_verifies_every_chunk_with_its_extracted_path` in `integration_test.rs`, restated in this node's terms: the indexes are every index below the reference content's chunk count under each concrete's declared chunk length.
- `a_challenge_response_verifies_against_the_root` (`hashing/blake3_bao`): absorbed; proven by `every_concrete_answers_a_challenge_with_a_response_that_verifies` in `integration_test.rs`, restated in this node's terms: the altered outboard has one byte inverted in each of the two child values the root's parent node records, so the challenged chunk's topmost sibling is altered whichever side of the root's split the chunk lies on.
- `keyed_paths_verify_only_under_their_key` (`hashing/blake3_bao`): absorbed; proven by `every_concrete_verifies_a_keyed_path_only_under_its_key` in this module's private integration test `integration_test.rs`, which runs the entry through `create_commitment` over the declared set and the mock selection.

## Integration: own entries

The four entries proven inside the crate take a private integration block of the same name in `integration_test.rs`; the entry the published vector and the independent oracle decide takes a public integration block of the same name in `tests/factory_integration_test.rs`.

- `every_concrete_is_constructed_under_the_identifier_it_declares`: condition `create_commitment` is called with the selection of a concrete and the identifier that concrete declares; outcome `Ok(CreateCommitmentSuccessReturn { adapter })` with `adapter.declaration()` reporting the identifier required and the interface version `COMMITMENT_INTERFACE_VERSION`; variation each case of the declared set and the mock selection, whose declared identifiers differ, so a factory that constructs a concrete other than the selected one reports another identifier; edge that must survive the composition: every concrete the factory admits declares the interface version the family's consumers are written against.
- `every_concrete_is_refused_under_an_identifier_it_does_not_declare`: condition `create_commitment` is called with the selection of a concrete and an identifier that concrete does not declare; outcome `Err(CreateCommitmentErrorReturn::UnsupportedCommitmentScheme)`; variation each case paired with the declared identifier of every other case whose declared identifier differs from its own, so an admission that compares against a constant or ignores the selection admits a pairing it must refuse; edge that must survive the composition: the refusal is decided by the selected concrete's own declaration.
- `every_concrete_returns_its_own_error_when_a_content_length_cannot_be_represented`: condition the adapter `create_commitment` returns for the mock selection under the failure mode `ContentLengthUnrepresentable` commits the reference content unkeyed; outcome `Err(CommitErrorReturn::Mock(MockICommitmentAdapterErrorReturn::ContentLengthUnrepresentable { length }))` with `length` equal to `REFERENCE_CONTENT_LENGTH`; variation the mock selection under that failure mode, the case that reaches this arm, since a `usize` byte length fits a `u64` on every supported target and no input takes it for a real concrete; edge that must survive the composition: the failure mode the configuration names is the failure mode of the concrete the factory constructs, and the concrete's own error arrives whole in the family's error union.
- `every_concrete_returns_its_own_error_when_an_offset_cannot_be_represented`: condition the adapter `create_commitment` returns for the mock selection under the failure mode `OffsetUnrepresentable` commits the reference content unkeyed; outcome `Err(CommitErrorReturn::Mock(MockICommitmentAdapterErrorReturn::OffsetUnrepresentable { offset }))` with `offset` equal to `REFERENCE_CONTENT_LENGTH` as a `u64`, the content's length, since `commit`'s payload carries no chunk index; variation the mock selection under that failure mode, the case that reaches this arm; edge that must survive the composition: as the entry above.
- `every_concrete_declaring_the_blake3_bao_identifier_commits_the_reference_content_to_the_bao_hash_and_outboard`: condition `create_commitment` is called with the selection of a concrete that declares `CommitmentSchemeIdentifier::Blake3BaoV1` and that identifier, and the adapter commits the reference content unkeyed; outcome `Ok(CommitSuccessReturn { root, outboard })` with `root.as_bytes()` equal to bao's published 3073-byte hash `5ba075072daba2470558a171e3769fba057dc3f12375c60892bdbe73348d9fd1` and `outboard.bytes` equal to `expected_outboard` over the reference content unkeyed; variation each case of the declared set that declares the identifier; edge that must survive the composition: the concrete the factory constructs commits, through the family's trait object, to the values the published vector and the independent oracle state.

## Integration: private surface

The chain of real functions is `create_commitment`, the selected concrete's `try_new`, and `declaration`, `commit`, `verify`, `extract_path`, `verify_chunk`, `challenge`, and `respond` as called on the returned adapter, with `Secret::expose` of `domain` running for real; the cases are the declared set and the mock selection; no outer-edge collaborator is mocked — `blake3`, `bao`, `zeroize`, and `domain` run as themselves; the observable result is the `Result` `create_commitment` returns, the declaration, each method's return, and the borrowed key's bytes after the call; the entries proven are `every_concrete_is_constructed_under_the_identifier_it_declares`, `every_concrete_is_refused_under_an_identifier_it_does_not_declare`, `every_concrete_returns_its_own_error_when_a_content_length_cannot_be_represented`, `every_concrete_returns_its_own_error_when_an_offset_cannot_be_represented`, `every_concrete_leaves_the_borrowed_key_intact`, `every_concrete_verifies_the_content_it_committed`, `every_concrete_verifies_every_chunk_with_its_extracted_path`, `every_concrete_answers_a_challenge_with_a_response_that_verifies`, and `every_concrete_verifies_a_keyed_path_only_under_its_key`.

## Integration: public surface

An outside caller invokes `create_commitment` with `CreateCommitmentDeps`, a `CreateCommitmentParams` taken from `DECLARED_COMMITMENT_SELECTIONS`, and a `CreateCommitmentPayload`, calls `commit` on the returned adapter with `CommitParams` and `CommitPayload`, and observes the root's bytes and the outboard's bytes; the entry proven is `every_concrete_declaring_the_blake3_bao_identifier_commits_the_reference_content_to_the_bao_hash_and_outboard`.
