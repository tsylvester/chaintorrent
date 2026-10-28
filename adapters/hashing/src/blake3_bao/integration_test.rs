#![allow(clippy::expect_used)]

use super::provides::build_blake3_bao_commitment;
use crate::factory::provides::{
    ChallengePayloadOverrides, ChunkOverrides, CommitmentKey, ExtractPathPayloadOverrides,
    ICommitmentAdapter, OutboardOverrides, REFERENCE_CONTENT_LENGTH, RespondPayloadOverrides,
    VerifyChunkErrorReturn, VerifyChunkPayloadOverrides, VerifyChunkSuccessReturn,
    VerifyErrorReturn, VerifySuccessReturn, build_challenge_params, build_challenge_payload,
    build_chunk, build_commit_params, build_commit_payload, build_extract_path_params,
    build_extract_path_payload, build_outboard, build_respond_params, build_respond_payload,
    build_verify_chunk_params, build_verify_chunk_payload, build_verify_params,
    build_verify_payload, reference_content, reference_key_bytes,
};
use domain::{SecretConstructorParamsOverrides, build_secret};

/// Contract: `commit_under_a_key_leaves_the_borrowed_key_intact`: `commit` is
///   called with `CommitmentKey::Keyed` borrowing a living `Secret<Vec<u8>>` →
///   after the call the secret's `expose()` equals the bytes it held before the
///   call, so the zeroization of the key copy and the keyed hashers wipes only
///   their own copies.
/// Arrange: `build_secret` over `reference_key_bytes()`, whose bytes are not
///   all zero; the reference content.
/// Act:     `adapter.commit` with `CommitmentKey::Keyed` borrowing the secret.
/// Assert:  the success arm binds through `expect`; `secret.expose()` equals
///   `reference_key_bytes()` by `assert_eq!`.
/// Boundary: `Blake3BaoCommitment::commit` through `build_blake3_bao_commitment`,
///   running `blake3`'s keyed hashers, `Zeroize::zeroize`, and `Root::try_new`.
/// Mocked:  nothing.
#[test]
fn commit_under_a_key_leaves_the_borrowed_key_intact() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let secret = build_secret(reference_key_bytes(), SecretConstructorParamsOverrides);
    let content = reference_content(REFERENCE_CONTENT_LENGTH);
    let payload = build_commit_payload(&content, CommitmentKey::Keyed(&secret), Default::default());

    // Act
    let result = adapter.commit(build_commit_params(Default::default()), payload);

    // Assert
    let _success = result.expect("the keyed commit is admitted");
    assert_eq!(secret.expose(), &reference_key_bytes());
}

/// Contract: `committed_outboard_verifies_its_content`: `verify` is given the
///   content, root, and outboard `commit` returned for it →
///   `Ok(VerifySuccessReturn)`, while the same root and outboard with one
///   content byte changed in the third chunk return
///   `Err(VerifyErrorReturn::ChunkMismatch { chunk_index: 2 })`.
/// Arrange: the reference content of 3073 bytes, four chunks under three parent
///   nodes, committed unkeyed; a copy with one byte of chunk 2 inverted.
/// Act:     `adapter.verify` over each with the committed root and outboard.
/// Assert:  the intact content returns `Ok(VerifySuccessReturn)` by
///   `assert!(matches!(…))`; the changed content returns the whole
///   `ChunkMismatch` error by `assert_eq!`.
/// Boundary: `commit` then `verify`, with `bao::encode::outboard` and
///   `blake3::hazmat` running for real.
/// Mocked:  nothing.
#[test]
fn committed_outboard_verifies_its_content() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let content = reference_content(REFERENCE_CONTENT_LENGTH);
    let committed = adapter
        .commit(
            build_commit_params(Default::default()),
            build_commit_payload(&content, CommitmentKey::Unkeyed, Default::default()),
        )
        .expect("the unkeyed commit is admitted");
    let mut altered = content.clone();
    altered[2048] ^= 0xff;

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

/// Contract: `extracted_paths_verify_every_chunk`: `verify_chunk` is given, for
///   every chunk index of a committed object, that chunk's bytes and the path
///   `extract_path` returned for that index → every call returns
///   `Ok(VerifyChunkSuccessReturn)`, while chunk 1's path with chunk 0's bytes
///   of the same length returns `Err(VerifyChunkErrorReturn::RootMismatch)`.
/// Arrange: the reference content of 3073 bytes committed unkeyed; its chunks
///   differ, so a path that ignores the index fails.
/// Act:     for indexes 0 through 3, `extract_path` then `verify_chunk` over
///   that chunk; then `verify_chunk` with chunk 1's path and chunk 0's bytes at
///   index 1.
/// Assert:  every indexed call returns `Ok(VerifyChunkSuccessReturn)` by
///   `assert!(matches!(…))`; the mismatched call returns `RootMismatch` by
///   `assert_eq!`.
/// Boundary: `commit`, `extract_path`, and `verify_chunk`.
/// Mocked:  nothing.
#[test]
fn extracted_paths_verify_every_chunk() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let content = reference_content(REFERENCE_CONTENT_LENGTH);
    let committed = adapter
        .commit(
            build_commit_params(Default::default()),
            build_commit_payload(&content, CommitmentKey::Unkeyed, Default::default()),
        )
        .expect("the unkeyed commit is admitted");

    // Act
    let mut results = Vec::new();
    for index in 0..4u64 {
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
        let start = usize::try_from(index * 1024).expect("a chunk offset fits a usize");
        let end = start + 1024usize.min(REFERENCE_CONTENT_LENGTH - start);
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
    let chunk_zero = build_chunk(ChunkOverrides {
        bytes: Some(content[0..1024].to_vec()),
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
    assert_eq!(mismatched.err(), Some(VerifyChunkErrorReturn::RootMismatch),);
}

/// Contract: `a_challenge_response_verifies_against_the_root`: `respond`
///   answers the chunk index `challenge` chose from fixed uniform bytes, over
///   the committed content and outboard, and `verify_chunk` checks the response
///   against the committed root → `Ok(VerifyChunkSuccessReturn)`, while a
///   response computed over an outboard with one parent byte changed returns
///   `Err(VerifyChunkErrorReturn::RootMismatch)`.
/// Arrange: the reference content committed unkeyed; the uniform bytes the
///   little-endian bytes of 6; a copy of the outboard with one byte of its
///   first parent node's left child inverted — the false-possession case of
///   SW-04.
/// Act:     `challenge`, then `respond` over the intact and the altered
///   outboard, then `verify_chunk` of each response against the committed root.
/// Assert:  the intact response returns `Ok(VerifyChunkSuccessReturn)` by
///   `assert!(matches!(…))`; the altered response returns `RootMismatch` by
///   `assert_eq!`.
/// Boundary: `commit`, `challenge`, `respond`, and `verify_chunk`.
/// Mocked:  nothing.
#[test]
fn a_challenge_response_verifies_against_the_root() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let content = reference_content(REFERENCE_CONTENT_LENGTH);
    let committed = adapter
        .commit(
            build_commit_params(Default::default()),
            build_commit_payload(&content, CommitmentKey::Unkeyed, Default::default()),
        )
        .expect("the unkeyed commit is admitted");
    let mut altered = committed.outboard.bytes.clone();
    altered[8] ^= 0xff;
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

/// Contract: `keyed_paths_verify_only_under_their_key`: a path extracted from a
///   keyed commitment is checked with `verify_chunk` under the same key and
///   unkeyed → `Ok(VerifyChunkSuccessReturn)` under the key and
///   `Err(VerifyChunkErrorReturn::RootMismatch)` unkeyed.
/// Arrange: `build_secret` over `reference_key_bytes()`; the reference content
///   committed under the key; chunk 2 and its extracted path.
/// Act:     `verify_chunk` under the key and unkeyed.
/// Assert:  under the key `Ok(VerifyChunkSuccessReturn)` by
///   `assert!(matches!(…))`; unkeyed `RootMismatch` by `assert_eq!`.
/// Boundary: keyed `commit`, `extract_path`, and `verify_chunk`.
/// Mocked:  nothing.
#[test]
fn keyed_paths_verify_only_under_their_key() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let secret = build_secret(reference_key_bytes(), SecretConstructorParamsOverrides);
    let content = reference_content(REFERENCE_CONTENT_LENGTH);
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
    let chunk = build_chunk(ChunkOverrides {
        bytes: Some(content[2048..3072].to_vec()),
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
