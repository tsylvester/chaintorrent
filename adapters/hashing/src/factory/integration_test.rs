#![allow(clippy::expect_used)]

use super::provides::{
    COMMITMENT_INTERFACE_VERSION, ChallengePayloadOverrides, ChunkOverrides, CommitErrorReturn,
    CommitmentConcrete, CommitmentKey, CreateCommitmentErrorReturn,
    CreateCommitmentParamsOverrides, DECLARED_COMMITMENT_SELECTIONS, ExtractPathPayloadOverrides,
    ICommitmentAdapter, MockICommitmentAdapterErrorReturn, MockICommitmentAdapterFailureMode,
    OutboardOverrides, REFERENCE_CONTENT_LENGTH, RespondPayloadOverrides, VerifyChunkErrorReturn,
    VerifyChunkPayloadOverrides, VerifyChunkSuccessReturn, VerifyErrorReturn, VerifySuccessReturn,
    build_challenge_params, build_challenge_payload, build_chunk, build_commit_params,
    build_commit_payload, build_create_commitment_deps, build_create_commitment_params,
    build_create_commitment_payload, build_extract_path_params, build_extract_path_payload,
    build_outboard, build_respond_params, build_respond_payload, build_verify_chunk_params,
    build_verify_chunk_payload, build_verify_params, build_verify_payload, create_commitment,
    reference_content, reference_key_bytes,
};
use domain::{SecretConstructorParamsOverrides, build_secret};

/// Contract: `create_commitment` is called with the selection of a
///   concrete and the identifier that concrete declares →
///   `Ok(CreateCommitmentSuccessReturn { adapter })` with
///   `adapter.declaration()` reporting the identifier required and the
///   interface version `COMMITMENT_INTERFACE_VERSION`.
/// Arrange: for each case, `build_create_commitment_deps`,
///   `build_create_commitment_params` overriding `concrete` and
///   `identifier` with the case's selection and identifier, and
///   `build_create_commitment_payload`; the cases declare different
///   identifiers, so a factory that constructs a concrete other than the
///   selected one reports another identifier.
/// Act:     `create_commitment(&deps, params, payload)` for the case.
/// Assert:  `adapter.declaration().identifier` equals the case's
///   identifier by `assert_eq!`, and `adapter.declaration().interface_version`
///   equals `COMMITMENT_INTERFACE_VERSION` by `assert_eq!`.
/// Boundary: the crate-internal path `create_commitment` through the
///   selected concrete's `try_new`, with the declaration read through the
///   returned adapter.
/// Mocked:  nothing; the declared concretes and the mock concrete run as
///   themselves, so this block does not prove a commitment.
#[test]
fn every_concrete_is_constructed_under_the_identifier_it_declares() {
    // Arrange
    let mock_selection = build_create_commitment_params(Default::default());
    for selection in DECLARED_COMMITMENT_SELECTIONS
        .iter()
        .chain(core::iter::once(&mock_selection))
    {
        let deps = build_create_commitment_deps(Default::default());
        let params = build_create_commitment_params(CreateCommitmentParamsOverrides {
            concrete: Some(selection.concrete),
            identifier: Some(selection.identifier),
        });
        let payload = build_create_commitment_payload(Default::default());

        // Act
        let result = create_commitment(&deps, params, payload);

        // Assert
        let success = result.expect("an admitted selection returns the adapter");
        let adapter: &dyn ICommitmentAdapter = &*success.adapter;
        assert_eq!(adapter.declaration().identifier, selection.identifier);
        assert_eq!(
            adapter.declaration().interface_version,
            COMMITMENT_INTERFACE_VERSION
        );
    }
}

/// Contract: `create_commitment` is called with the selection of a
///   concrete and an identifier that concrete does not declare →
///   `Err(CreateCommitmentErrorReturn::UnsupportedCommitmentScheme)`.
/// Arrange: for each pair of cases whose declared identifiers differ,
///   `build_create_commitment_deps`,
///   `build_create_commitment_params` overriding `concrete` with the
///   first case's selection and `identifier` with the second case's
///   identifier, and `build_create_commitment_payload`; every pairing
///   carries an identifier the selected concrete does not declare, so an
///   admission that compares against a constant or ignores the selection
///   admits a pairing it must refuse.
/// Act:     `create_commitment(&deps, params, payload)` for the pairing.
/// Assert:  `result.err()` equals
///   `Some(CreateCommitmentErrorReturn::UnsupportedCommitmentScheme)` by
///   `assert_eq!`.
/// Boundary: the crate-internal path `create_commitment`, whose
///   admission reads the selected concrete's declaration.
/// Mocked:  nothing.
#[test]
fn every_concrete_is_refused_under_an_identifier_it_does_not_declare() {
    // Arrange
    let mock_selection = build_create_commitment_params(Default::default());
    for case in DECLARED_COMMITMENT_SELECTIONS
        .iter()
        .chain(core::iter::once(&mock_selection))
    {
        for other in DECLARED_COMMITMENT_SELECTIONS
            .iter()
            .chain(core::iter::once(&mock_selection))
        {
            if other.identifier == case.identifier {
                continue;
            }
            let deps = build_create_commitment_deps(Default::default());
            let params = build_create_commitment_params(CreateCommitmentParamsOverrides {
                concrete: Some(case.concrete),
                identifier: Some(other.identifier),
            });
            let payload = build_create_commitment_payload(Default::default());

            // Act
            let result = create_commitment(&deps, params, payload);

            // Assert
            assert_eq!(
                result.err(),
                Some(CreateCommitmentErrorReturn::UnsupportedCommitmentScheme)
            );
        }
    }
}

/// Contract: the adapter `create_commitment` returns for the mock
///   selection under the failure mode `ContentLengthUnrepresentable`
///   commits the reference content unkeyed →
///   `Err(CommitErrorReturn::Mock(MockICommitmentAdapterErrorReturn::ContentLengthUnrepresentable { length }))`
///   with `length` equal to `REFERENCE_CONTENT_LENGTH`.
/// Arrange: the case is the mock selection alone, which the declared set
///   cannot reach; `build_create_commitment_params` with the `concrete`
///   override
///   `CommitmentConcrete::Mock(MockICommitmentAdapterFailureMode::ContentLengthUnrepresentable)`;
///   the adapter from `create_commitment` over `build_create_commitment_deps`,
///   those params, and `build_create_commitment_payload`, bound through
///   `expect`; `build_commit_params` with no override and
///   `build_commit_payload` over the reference content with
///   `CommitmentKey::Unkeyed`.
/// Act:     `adapter.commit(params, payload)`.
/// Assert:  `result.err()` equals
///   `Some(CommitErrorReturn::Mock(MockICommitmentAdapterErrorReturn::ContentLengthUnrepresentable { length: REFERENCE_CONTENT_LENGTH }))`
///   by `assert_eq!`.
/// Boundary: the crate-internal path `create_commitment` through the
///   mock concrete's `try_new` with the configured failure mode, then
///   the returned adapter's `commit`.
/// Mocked:  nothing.
#[test]
fn every_concrete_returns_its_own_error_when_a_content_length_cannot_be_represented() {
    // Arrange
    let deps = build_create_commitment_deps(Default::default());
    let params = build_create_commitment_params(CreateCommitmentParamsOverrides {
        concrete: Some(CommitmentConcrete::Mock(
            MockICommitmentAdapterFailureMode::ContentLengthUnrepresentable,
        )),
        ..Default::default()
    });
    let payload = build_create_commitment_payload(Default::default());
    let success =
        create_commitment(&deps, params, payload).expect("the mock selection is admitted");
    let adapter: &dyn ICommitmentAdapter = &*success.adapter;
    let content = reference_content(REFERENCE_CONTENT_LENGTH);
    let commit_payload = build_commit_payload(&content, CommitmentKey::Unkeyed, Default::default());

    // Act
    let result = adapter.commit(build_commit_params(Default::default()), commit_payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(CommitErrorReturn::Mock(
            MockICommitmentAdapterErrorReturn::ContentLengthUnrepresentable {
                length: REFERENCE_CONTENT_LENGTH,
            }
        ))
    );
}

/// Contract: the adapter `create_commitment` returns for the mock
///   selection under the failure mode `OffsetUnrepresentable` commits
///   the reference content unkeyed →
///   `Err(CommitErrorReturn::Mock(MockICommitmentAdapterErrorReturn::OffsetUnrepresentable { offset }))`
///   with `offset` equal to `REFERENCE_CONTENT_LENGTH` as a `u64`, the
///   content's length, since `commit`'s payload carries no chunk index.
/// Arrange: the case is the mock selection alone, which the declared set
///   cannot reach; `build_create_commitment_params` with the `concrete`
///   override
///   `CommitmentConcrete::Mock(MockICommitmentAdapterFailureMode::OffsetUnrepresentable)`;
///   the adapter from `create_commitment` over `build_create_commitment_deps`,
///   those params, and `build_create_commitment_payload`, bound through
///   `expect`; `build_commit_params` with no override and
///   `build_commit_payload` over the reference content with
///   `CommitmentKey::Unkeyed`.
/// Act:     `adapter.commit(params, payload)`.
/// Assert:  `result.err()` equals
///   `Some(CommitErrorReturn::Mock(MockICommitmentAdapterErrorReturn::OffsetUnrepresentable { offset }))`
///   by `assert_eq!`, with `offset` `REFERENCE_CONTENT_LENGTH` converted
///   through `u64::try_from` bound by `expect`.
/// Boundary: the crate-internal path `create_commitment` through the
///   mock concrete's `try_new` with the configured failure mode, then
///   the returned adapter's `commit`.
/// Mocked:  nothing.
#[test]
fn every_concrete_returns_its_own_error_when_an_offset_cannot_be_represented() {
    // Arrange
    let deps = build_create_commitment_deps(Default::default());
    let params = build_create_commitment_params(CreateCommitmentParamsOverrides {
        concrete: Some(CommitmentConcrete::Mock(
            MockICommitmentAdapterFailureMode::OffsetUnrepresentable,
        )),
        ..Default::default()
    });
    let payload = build_create_commitment_payload(Default::default());
    let success =
        create_commitment(&deps, params, payload).expect("the mock selection is admitted");
    let adapter: &dyn ICommitmentAdapter = &*success.adapter;
    let content = reference_content(REFERENCE_CONTENT_LENGTH);
    let commit_payload = build_commit_payload(&content, CommitmentKey::Unkeyed, Default::default());

    // Act
    let result = adapter.commit(build_commit_params(Default::default()), commit_payload);

    // Assert
    let offset =
        u64::try_from(REFERENCE_CONTENT_LENGTH).expect("the reference content length fits a u64");
    assert_eq!(
        result.err(),
        Some(CommitErrorReturn::Mock(
            MockICommitmentAdapterErrorReturn::OffsetUnrepresentable { offset }
        ))
    );
}

/// Contract: the absorbed entry `hashing/blake3_bao`
///   `commit_under_a_key_leaves_the_borrowed_key_intact`, in this node's
///   terms: `commit` is called on the adapter `create_commitment`
///   returns with `CommitmentKey::Keyed` borrowing a living
///   `Secret<Vec<u8>>` → after the call the secret's `expose()` equals
///   the bytes it held before the call.
/// Arrange: `build_secret` with the value `reference_key_bytes()`, whose
///   bytes are not all zero, so a zeroized secret differs from an intact
///   one; for each case, the adapter from `create_commitment` over
///   `build_create_commitment_deps`, the case's params, and
///   `build_create_commitment_payload`, bound through `expect`;
///   `build_commit_payload` over the reference content with
///   `CommitmentKey::Keyed` borrowing that secret.
/// Act:     `adapter.commit(build_commit_params(Default::default()), payload)`
///   for the case.
/// Assert:  the success arm is bound through `expect`; the secret's
///   `expose()` equals `reference_key_bytes()` by `assert_eq!`.
/// Boundary: the crate-internal path `create_commitment` through the
///   selected concrete's `try_new`, then the returned adapter's keyed
///   `commit`, with `Secret::expose` reading the key.
/// Mocked:  nothing; `blake3`, `bao`, `zeroize`, and `domain` run for
///   real.
#[test]
fn every_concrete_leaves_the_borrowed_key_intact() {
    // Arrange
    let secret = build_secret(reference_key_bytes(), SecretConstructorParamsOverrides);
    let content = reference_content(REFERENCE_CONTENT_LENGTH);
    let mock_selection = build_create_commitment_params(Default::default());
    for selection in DECLARED_COMMITMENT_SELECTIONS
        .iter()
        .chain(core::iter::once(&mock_selection))
    {
        let deps = build_create_commitment_deps(Default::default());
        let payload = build_create_commitment_payload(Default::default());
        let success = create_commitment(&deps, *selection, payload)
            .expect("an admitted selection returns the adapter");
        let adapter: &dyn ICommitmentAdapter = &*success.adapter;
        let commit_payload =
            build_commit_payload(&content, CommitmentKey::Keyed(&secret), Default::default());

        // Act
        let result = adapter.commit(build_commit_params(Default::default()), commit_payload);

        // Assert
        let _success = result.expect("the keyed commit is admitted");
        assert_eq!(secret.expose(), &reference_key_bytes());
    }
}

/// Contract: the absorbed entry `hashing/blake3_bao`
///   `committed_outboard_verifies_its_content`, in this node's terms:
///   `verify` is given the content, root, and outboard `commit` returned
///   for it → `Ok(VerifySuccessReturn)`, while the same root and
///   outboard over the content with the byte at offset `2 × L`
///   inverted → `Err(VerifyErrorReturn::ChunkMismatch { chunk_index: 2 })`.
/// Arrange: for each case, the adapter from `create_commitment` over
///   `build_create_commitment_deps`, the case's params, and
///   `build_create_commitment_payload`, bound through `expect`; the
///   reference content committed unkeyed, the success arm bound through
///   `expect`; a copy of the content with the byte at offset `2 × L`
///   inverted, `L` read from the case's declaration, so the changed
///   chunk is chunk 2 under each concrete's own chunk length.
/// Act:     `adapter.verify` over the intact content and over the
///   changed copy, each with the committed root and outboard and
///   `CommitmentKey::Unkeyed`.
/// Assert:  the intact content's result matches `Ok(VerifySuccessReturn)`
///   by `assert!(matches!(…))`; the changed copy's result equals
///   `Err(VerifyErrorReturn::ChunkMismatch { chunk_index: 2 })` by
///   `assert_eq!`.
/// Boundary: the crate-internal path `create_commitment`, then `commit`
///   and `verify` on the returned adapter.
/// Mocked:  nothing.
#[test]
fn every_concrete_verifies_the_content_it_committed() {
    // Arrange
    let content = reference_content(REFERENCE_CONTENT_LENGTH);
    let mock_selection = build_create_commitment_params(Default::default());
    for selection in DECLARED_COMMITMENT_SELECTIONS
        .iter()
        .chain(core::iter::once(&mock_selection))
    {
        let deps = build_create_commitment_deps(Default::default());
        let payload = build_create_commitment_payload(Default::default());
        let success = create_commitment(&deps, *selection, payload)
            .expect("an admitted selection returns the adapter");
        let adapter: &dyn ICommitmentAdapter = &*success.adapter;
        let committed = adapter
            .commit(
                build_commit_params(Default::default()),
                build_commit_payload(&content, CommitmentKey::Unkeyed, Default::default()),
            )
            .expect("the unkeyed commit is admitted");
        let chunk_length = usize::try_from(adapter.declaration().chunk_length)
            .expect("a chunk length fits a usize");
        let mut altered = content.clone();
        altered[2 * chunk_length] ^= 0xff;

        // Act
        let intact = adapter.verify(
            build_verify_params(Default::default()),
            build_verify_payload(
                &content,
                &committed.outboard,
                &committed.root,
                CommitmentKey::Unkeyed,
                Default::default(),
            ),
        );
        let changed = adapter.verify(
            build_verify_params(Default::default()),
            build_verify_payload(
                &altered,
                &committed.outboard,
                &committed.root,
                CommitmentKey::Unkeyed,
                Default::default(),
            ),
        );

        // Assert
        assert!(matches!(intact, Ok(VerifySuccessReturn)));
        assert_eq!(
            changed.err(),
            Some(VerifyErrorReturn::ChunkMismatch { chunk_index: 2 }),
        );
    }
}

/// Contract: the absorbed entry `hashing/blake3_bao`
///   `extracted_paths_verify_every_chunk`, in this node's terms: for
///   every index below the reference content's chunk count,
///   `verify_chunk` given that chunk and the path `extract_path`
///   returned for its index → `Ok(VerifyChunkSuccessReturn)`, while
///   chunk 1's path with chunk 0's bytes at index 1 →
///   `Err(VerifyChunkErrorReturn::RootMismatch)`.
/// Arrange: for each case, the adapter from `create_commitment` over
///   `build_create_commitment_deps`, the case's params, and
///   `build_create_commitment_payload`, bound through `expect`; the
///   reference content committed unkeyed; for each index below its
///   chunk count, `build_extract_path_payload` over the committed
///   outboard with that index, and `build_chunk` with that chunk's
///   bytes; `build_chunk` with chunk 0's bytes for the mismatched call,
///   chunks 0 and 1 being full chunks of different bytes under every
///   concrete.
/// Act:     `extract_path` then `verify_chunk` for each index, with
///   `build_verify_chunk_payload` over the committed root, the chunk,
///   the path, and `CommitmentKey::Unkeyed`, the content length the
///   reference length and the index the chunk's; then `verify_chunk`
///   with chunk 1's path and chunk 0's bytes at index 1.
/// Assert:  every indexed call matches `Ok(VerifyChunkSuccessReturn)`
///   by `assert!(matches!(…))`; the mismatched call equals
///   `Err(VerifyChunkErrorReturn::RootMismatch)` by `assert_eq!`.
/// Boundary: the crate-internal path `create_commitment`, then `commit`,
///   `extract_path`, and `verify_chunk` on the returned adapter.
/// Mocked:  nothing.
#[test]
fn every_concrete_verifies_every_chunk_with_its_extracted_path() {
    // Arrange
    let content = reference_content(REFERENCE_CONTENT_LENGTH);
    let mock_selection = build_create_commitment_params(Default::default());
    for selection in DECLARED_COMMITMENT_SELECTIONS
        .iter()
        .chain(core::iter::once(&mock_selection))
    {
        let deps = build_create_commitment_deps(Default::default());
        let payload = build_create_commitment_payload(Default::default());
        let success = create_commitment(&deps, *selection, payload)
            .expect("an admitted selection returns the adapter");
        let adapter: &dyn ICommitmentAdapter = &*success.adapter;
        let committed = adapter
            .commit(
                build_commit_params(Default::default()),
                build_commit_payload(&content, CommitmentKey::Unkeyed, Default::default()),
            )
            .expect("the unkeyed commit is admitted");
        let chunk_length = adapter.declaration().chunk_length;
        let chunk_count = u64::try_from(REFERENCE_CONTENT_LENGTH)
            .expect("the reference content length fits a u64")
            .div_ceil(chunk_length);

        // Act
        let mut results = Vec::new();
        for index in 0..chunk_count {
            let path = adapter
                .extract_path(
                    build_extract_path_params(Default::default()),
                    build_extract_path_payload(
                        &committed.outboard,
                        ExtractPathPayloadOverrides {
                            chunk_index: Some(index),
                        },
                    ),
                )
                .expect("the index is in range")
                .path;
            let start = usize::try_from(index * chunk_length).expect("a chunk offset fits a usize");
            let end = start
                + usize::try_from(chunk_length)
                    .expect("a chunk length fits a usize")
                    .min(REFERENCE_CONTENT_LENGTH - start);
            let chunk = build_chunk(ChunkOverrides {
                bytes: Some(content[start..end].to_vec()),
            });
            results.push(adapter.verify_chunk(
                build_verify_chunk_params(Default::default()),
                build_verify_chunk_payload(
                    &committed.root,
                    &chunk,
                    &path,
                    CommitmentKey::Unkeyed,
                    VerifyChunkPayloadOverrides {
                        chunk_index: Some(index),
                        ..Default::default()
                    },
                ),
            ));
        }
        let path_one = adapter
            .extract_path(
                build_extract_path_params(Default::default()),
                build_extract_path_payload(
                    &committed.outboard,
                    ExtractPathPayloadOverrides {
                        chunk_index: Some(1),
                    },
                ),
            )
            .expect("the index is in range")
            .path;
        let end_zero = usize::try_from(chunk_length)
            .expect("a chunk length fits a usize")
            .min(REFERENCE_CONTENT_LENGTH);
        let chunk_zero = build_chunk(ChunkOverrides {
            bytes: Some(content[0..end_zero].to_vec()),
        });
        let mismatched = adapter.verify_chunk(
            build_verify_chunk_params(Default::default()),
            build_verify_chunk_payload(
                &committed.root,
                &chunk_zero,
                &path_one,
                CommitmentKey::Unkeyed,
                VerifyChunkPayloadOverrides {
                    chunk_index: Some(1),
                    ..Default::default()
                },
            ),
        );

        // Assert
        for result in results {
            assert!(matches!(result, Ok(VerifyChunkSuccessReturn)));
        }
        assert_eq!(mismatched.err(), Some(VerifyChunkErrorReturn::RootMismatch));
    }
}

/// Contract: the absorbed entry `hashing/blake3_bao`
///   `a_challenge_response_verifies_against_the_root`, in this node's
///   terms: `respond` answers the chunk index `challenge` chose from
///   fixed uniform bytes over the committed content and outboard, and
///   `verify_chunk` checks the response against the committed root →
///   `Ok(VerifyChunkSuccessReturn)`, while a response computed over the
///   outboard with one byte inverted in each of the two child values
///   the root's parent node records → `Err(VerifyChunkErrorReturn::RootMismatch)`.
/// Arrange: for each case, the adapter from `create_commitment` over
///   `build_create_commitment_deps`, the case's params, and
///   `build_create_commitment_payload`, bound through `expect`; the
///   reference content committed unkeyed; `build_challenge_payload`
///   with the uniform override the little-endian bytes of 6; a copy of
///   the committed outboard built through `build_outboard` with the
///   bytes at offsets `8` and `40`, the first byte of each child value
///   of the root's parent node, inverted.
/// Act:     `challenge`, then `respond` at the chosen index over the
///   intact and the altered outboard, then `verify_chunk` of each
///   response against the committed root, unkeyed, at that index and
///   the reference content length.
/// Assert:  the intact response's result matches
///   `Ok(VerifyChunkSuccessReturn)` by `assert!(matches!(…))`; the
///   altered response's result equals
///   `Err(VerifyChunkErrorReturn::RootMismatch)` by `assert_eq!`.
/// Boundary: the crate-internal path `create_commitment`, then `commit`,
///   `challenge`, `respond`, and `verify_chunk` on the returned adapter.
/// Mocked:  nothing.
#[test]
fn every_concrete_answers_a_challenge_with_a_response_that_verifies() {
    // Arrange
    let content = reference_content(REFERENCE_CONTENT_LENGTH);
    let mock_selection = build_create_commitment_params(Default::default());
    for selection in DECLARED_COMMITMENT_SELECTIONS
        .iter()
        .chain(core::iter::once(&mock_selection))
    {
        let deps = build_create_commitment_deps(Default::default());
        let payload = build_create_commitment_payload(Default::default());
        let success = create_commitment(&deps, *selection, payload)
            .expect("an admitted selection returns the adapter");
        let adapter: &dyn ICommitmentAdapter = &*success.adapter;
        let committed = adapter
            .commit(
                build_commit_params(Default::default()),
                build_commit_payload(&content, CommitmentKey::Unkeyed, Default::default()),
            )
            .expect("the unkeyed commit is admitted");
        let mut altered = committed.outboard.bytes.clone();
        altered[8] ^= 0xff;
        altered[40] ^= 0xff;
        let altered_outboard = build_outboard(OutboardOverrides {
            bytes: Some(altered),
        });

        // Act
        let index = adapter
            .challenge(
                build_challenge_params(Default::default()),
                build_challenge_payload(ChallengePayloadOverrides {
                    uniform: Some(6u64.to_le_bytes()),
                    ..Default::default()
                }),
            )
            .expect("the challenge always succeeds")
            .chunk_index;
        let intact_response = adapter
            .respond(
                build_respond_params(Default::default()),
                build_respond_payload(
                    &content,
                    &committed.outboard,
                    RespondPayloadOverrides {
                        chunk_index: Some(index),
                    },
                ),
            )
            .expect("the intact response is admitted");
        let intact = adapter.verify_chunk(
            build_verify_chunk_params(Default::default()),
            build_verify_chunk_payload(
                &committed.root,
                &intact_response.chunk,
                &intact_response.path,
                CommitmentKey::Unkeyed,
                VerifyChunkPayloadOverrides {
                    chunk_index: Some(index),
                    ..Default::default()
                },
            ),
        );
        let altered_response = adapter
            .respond(
                build_respond_params(Default::default()),
                build_respond_payload(
                    &content,
                    &altered_outboard,
                    RespondPayloadOverrides {
                        chunk_index: Some(index),
                    },
                ),
            )
            .expect("the altered response is admitted");
        let altered_result = adapter.verify_chunk(
            build_verify_chunk_params(Default::default()),
            build_verify_chunk_payload(
                &committed.root,
                &altered_response.chunk,
                &altered_response.path,
                CommitmentKey::Unkeyed,
                VerifyChunkPayloadOverrides {
                    chunk_index: Some(index),
                    ..Default::default()
                },
            ),
        );

        // Assert
        assert!(matches!(intact, Ok(VerifyChunkSuccessReturn)));
        assert_eq!(
            altered_result.err(),
            Some(VerifyChunkErrorReturn::RootMismatch),
        );
    }
}

/// Contract: the absorbed entry `hashing/blake3_bao`
///   `keyed_paths_verify_only_under_their_key`: a path extracted from a
///   keyed commitment is checked with `verify_chunk` under the same key
///   and unkeyed → `Ok(VerifyChunkSuccessReturn)` under the key and
///   `Err(VerifyChunkErrorReturn::RootMismatch)` unkeyed.
/// Arrange: `build_secret` with the value `reference_key_bytes()`; for
///   each case, the adapter from `create_commitment` over
///   `build_create_commitment_deps`, the case's params, and
///   `build_create_commitment_payload`, bound through `expect`; the
///   reference content committed with `CommitmentKey::Keyed` borrowing
///   that secret; chunk 2 and the path `extract_path` returns for
///   index 2.
/// Act:     `verify_chunk` at index 2 over the committed root under
///   `CommitmentKey::Keyed` borrowing the secret, and under
///   `CommitmentKey::Unkeyed`.
/// Assert:  under the key the result matches `Ok(VerifyChunkSuccessReturn)`
///   by `assert!(matches!(…))`; unkeyed the result equals
///   `Err(VerifyChunkErrorReturn::RootMismatch)` by `assert_eq!`.
/// Boundary: the crate-internal path `create_commitment`, then keyed
///   `commit`, `extract_path`, and `verify_chunk` on the returned adapter.
/// Mocked:  nothing; `blake3`, `bao`, `zeroize`, and `domain` run for
///   real.
#[test]
fn every_concrete_verifies_a_keyed_path_only_under_its_key() {
    // Arrange
    let secret = build_secret(reference_key_bytes(), SecretConstructorParamsOverrides);
    let content = reference_content(REFERENCE_CONTENT_LENGTH);
    let mock_selection = build_create_commitment_params(Default::default());
    for selection in DECLARED_COMMITMENT_SELECTIONS
        .iter()
        .chain(core::iter::once(&mock_selection))
    {
        let deps = build_create_commitment_deps(Default::default());
        let payload = build_create_commitment_payload(Default::default());
        let success = create_commitment(&deps, *selection, payload)
            .expect("an admitted selection returns the adapter");
        let adapter: &dyn ICommitmentAdapter = &*success.adapter;
        let committed = adapter
            .commit(
                build_commit_params(Default::default()),
                build_commit_payload(&content, CommitmentKey::Keyed(&secret), Default::default()),
            )
            .expect("the keyed commit is admitted");
        let path = adapter
            .extract_path(
                build_extract_path_params(Default::default()),
                build_extract_path_payload(
                    &committed.outboard,
                    ExtractPathPayloadOverrides {
                        chunk_index: Some(2),
                    },
                ),
            )
            .expect("the index is in range")
            .path;
        let chunk_length = adapter.declaration().chunk_length;
        let start = usize::try_from(2 * chunk_length).expect("a chunk offset fits a usize");
        let end = start
            + usize::try_from(chunk_length)
                .expect("a chunk length fits a usize")
                .min(REFERENCE_CONTENT_LENGTH - start);
        let chunk = build_chunk(ChunkOverrides {
            bytes: Some(content[start..end].to_vec()),
        });

        // Act
        let keyed = adapter.verify_chunk(
            build_verify_chunk_params(Default::default()),
            build_verify_chunk_payload(
                &committed.root,
                &chunk,
                &path,
                CommitmentKey::Keyed(&secret),
                VerifyChunkPayloadOverrides {
                    chunk_index: Some(2),
                    ..Default::default()
                },
            ),
        );
        let unkeyed = adapter.verify_chunk(
            build_verify_chunk_params(Default::default()),
            build_verify_chunk_payload(
                &committed.root,
                &chunk,
                &path,
                CommitmentKey::Unkeyed,
                VerifyChunkPayloadOverrides {
                    chunk_index: Some(2),
                    ..Default::default()
                },
            ),
        );

        // Assert
        assert!(matches!(keyed, Ok(VerifyChunkSuccessReturn)));
        assert_eq!(unkeyed.err(), Some(VerifyChunkErrorReturn::RootMismatch));
    }
}
