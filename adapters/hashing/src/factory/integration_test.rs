#![allow(clippy::expect_used)]

use super::provides::{
    COMMITMENT_CONCRETES, ChallengePayloadOverrides, ChunkOverrides, CommitErrorReturn,
    CommitReturn, CommitmentConcrete, CommitmentConcreteOverrides, CommitmentKey,
    ConsumeCommitmentParams, ConsumeCommitmentPayload, CreateCommitmentDepsOverrides,
    CreateCommitmentParamsOverrides, ExtractPathPayloadOverrides, ICommitmentAdapter,
    ICommitmentConsumer, MockICommitmentAdapterErrorReturn, MockICommitmentAdapterFailureMode,
    OUTBOARD_HEADER_LENGTH, OUTBOARD_PARENT_LENGTH, OutboardOverrides, REFERENCE_CONTENT_LENGTH,
    RespondPayloadOverrides, VerifyChunkErrorReturn, VerifyChunkPayloadOverrides,
    VerifyChunkReturn, VerifyChunkSuccessReturn, VerifyErrorReturn, VerifyReturn,
    VerifySuccessReturn, build_challenge_params, build_challenge_payload, build_chunk,
    build_commit_params, build_commit_payload, build_commitment_concrete,
    build_create_commitment_deps, build_create_commitment_params, build_create_commitment_payload,
    build_extract_path_params, build_extract_path_payload, build_outboard, build_respond_params,
    build_respond_payload, build_verify_chunk_params, build_verify_chunk_payload,
    build_verify_params, build_verify_payload, create_commitment, reference_content,
    reference_key_bytes,
};
use crate::admit_hashing_composition::provides::admit_hashing_composition;
use crate::lookup_hashing_declaration::provides::{
    LookupHashingDeclarationParamsOverrides, build_lookup_hashing_declaration_deps,
    build_lookup_hashing_declaration_params, build_lookup_hashing_declaration_payload,
    lookup_hashing_declaration,
};
use domain::{Secret, SecretConstructorParamsOverrides, build_secret};

/// Contract: the own entry
///   `every_concrete_is_constructed_under_the_identifier_it_declares`, and
///   the absorbed entry `hashing/admit_hashing_composition`
///   `every_declared_member_declares_itself_through_the_public_surface`:
///   `create_commitment` with the case's selection and the identifier the
///   real lookup returns for it → `Ok(CreateCommitmentSuccessReturn { output })`
///   whose `output` is the `CommitReturn` the consumer observes on the
///   adapter it receives, the selected concrete.
/// Arrange: for each case, `build_create_commitment_params` overriding
///   `concrete` and `identifier` with the case's selection and identifier;
///   `build_commit_payload` over the reference content with
///   `CommitmentKey::Unkeyed`; the block's `Consumer`, whose
///   `consume_commitment` calls
///   `commit(build_commit_params(Default::default()), commit_payload)` on
///   the adapter it receives and returns the `CommitReturn` it observes;
///   the cases declare different chunk lengths, so the outboard length
///   separates the concretes.
/// Act:     `create_commitment(&deps, params, payload)` for the case.
/// Assert:  the success arms are bound through `expect`; the
///   `CommitSuccessReturn` each `output` observes is bound through
///   `expect`; `outboard.bytes.len()` equals
///   `OUTBOARD_HEADER_LENGTH + OUTBOARD_PARENT_LENGTH × (chunk count − 1)`
///   by `assert_eq!`, the chunk count `REFERENCE_CONTENT_LENGTH` as a `u64`
///   divided by `L`, rounded up, each term converted through `TryFrom`
///   bound by `expect`, the expectation computed from the lookup's
///   declaration and the layout constants and not from the factory.
/// Boundary: the crate-internal path `create_commitment` through the real
///   `admit_hashing_composition`, the real `lookup_hashing_declaration`,
///   and the selected concrete's `try_new`, then
///   `ICommitmentConsumer::consume_commitment` calling `commit` on the
///   adapter it receives.
/// Mocked:  nothing; the declared concretes and the mock concrete run as
///   themselves.
#[test]
fn every_concrete_is_constructed_under_the_identifier_it_declares() {
    struct Consumer;

    impl ICommitmentConsumer for Consumer {
        type Output = CommitReturn;

        fn consume_commitment<A: ICommitmentAdapter>(
            &self,
            _params: ConsumeCommitmentParams,
            payload: ConsumeCommitmentPayload<A>,
        ) -> Self::Output {
            let content = reference_content(REFERENCE_CONTENT_LENGTH);
            payload.adapter.commit(
                build_commit_params(Default::default()),
                build_commit_payload(&content, CommitmentKey::Unkeyed, Default::default()),
            )
        }
    }

    // Arrange
    let mock_concrete = CommitmentConcrete::Mock(MockICommitmentAdapterFailureMode::NoFailure);
    for concrete in COMMITMENT_CONCRETES
        .iter()
        .map(|member| build_commitment_concrete(*member, CommitmentConcreteOverrides))
        .chain(core::iter::once(mock_concrete))
    {
        let declaration = {
            let Ok(looked_up) = lookup_hashing_declaration(
                &build_lookup_hashing_declaration_deps(Default::default()),
                build_lookup_hashing_declaration_params(LookupHashingDeclarationParamsOverrides {
                    concrete: Some(concrete),
                }),
                build_lookup_hashing_declaration_payload(Default::default()),
            );
            looked_up.declaration
        };
        let deps = build_create_commitment_deps(
            Consumer,
            CreateCommitmentDepsOverrides {
                admit_hashing_composition: Some(admit_hashing_composition),
                lookup_hashing_declaration: Some(lookup_hashing_declaration),
            },
        );
        let params = build_create_commitment_params(CreateCommitmentParamsOverrides {
            concrete: Some(concrete),
            identifier: Some(declaration.identifier),
        });
        let payload = build_create_commitment_payload(Default::default());

        // Act
        let success = create_commitment(&deps, params, payload)
            .expect("an admitted selection returns the consumer's output");

        // Assert
        let committed = success.output.expect("the unkeyed commit is admitted");
        let chunk_count = u64::try_from(REFERENCE_CONTENT_LENGTH)
            .expect("the reference content length fits a u64")
            .div_ceil(declaration.chunk_length);
        let expected = OUTBOARD_HEADER_LENGTH
            + OUTBOARD_PARENT_LENGTH
                * usize::try_from(chunk_count - 1).expect("the chunk count fits a usize");
        assert_eq!(committed.outboard.bytes.len(), expected);
    }
}

/// Contract: the own entry
///   `every_concrete_returns_its_own_error_when_a_content_length_cannot_be_represented`:
///   `create_commitment` is called with the mock selection under the
///   failure mode `ContentLengthUnrepresentable` and a consumer whose
///   `consume_commitment` calls `commit` on the adapter it receives with
///   the reference content unkeyed → `Ok(CreateCommitmentSuccessReturn { output })`
///   whose `output` observes
///   `Err(CommitErrorReturn::Mock(MockICommitmentAdapterErrorReturn::ContentLengthUnrepresentable { length }))`
///   with `length` equal to `REFERENCE_CONTENT_LENGTH`.
/// Arrange: the case is the mock selection alone, which the declared set
///   cannot reach; `build_create_commitment_params` with the `concrete`
///   override
///   `CommitmentConcrete::Mock(MockICommitmentAdapterFailureMode::ContentLengthUnrepresentable)`
///   and the `identifier` the real `lookup_hashing_declaration` returns
///   for it; `build_commit_params` with no override and
///   `build_commit_payload` over the reference content with
///   `CommitmentKey::Unkeyed`; the block's `Consumer`, whose
///   `consume_commitment` calls `adapter.commit(params, payload)` on the
///   adapter it receives and returns the `CommitReturn` it observes as
///   its `Output`.
/// Act:     `create_commitment(&deps, params, payload)` over
///   `build_create_commitment_deps` holding the `Consumer`, the real
///   `admit_hashing_composition` and `lookup_hashing_declaration`, those
///   params, and `build_create_commitment_payload`.
/// Assert:  the success arm is bound through `expect`; the `output`'s
///   `err()` equals
///   `Some(CommitErrorReturn::Mock(MockICommitmentAdapterErrorReturn::ContentLengthUnrepresentable { length: REFERENCE_CONTENT_LENGTH }))`
///   by `assert_eq!`.
/// Boundary: the crate-internal path `create_commitment` through the
///   mock concrete's `try_new` with the configured failure mode, then
///   `ICommitmentConsumer::consume_commitment` calling `commit` on the
///   adapter it receives.
/// Mocked:  nothing.
#[test]
fn every_concrete_returns_its_own_error_when_a_content_length_cannot_be_represented() {
    struct Consumer;

    impl ICommitmentConsumer for Consumer {
        type Output = CommitReturn;

        fn consume_commitment<A: ICommitmentAdapter>(
            &self,
            _params: ConsumeCommitmentParams,
            payload: ConsumeCommitmentPayload<A>,
        ) -> Self::Output {
            let content = reference_content(REFERENCE_CONTENT_LENGTH);
            payload.adapter.commit(
                build_commit_params(Default::default()),
                build_commit_payload(&content, CommitmentKey::Unkeyed, Default::default()),
            )
        }
    }

    // Arrange
    let concrete =
        CommitmentConcrete::Mock(MockICommitmentAdapterFailureMode::ContentLengthUnrepresentable);
    let declaration = {
        let Ok(looked_up) = lookup_hashing_declaration(
            &build_lookup_hashing_declaration_deps(Default::default()),
            build_lookup_hashing_declaration_params(LookupHashingDeclarationParamsOverrides {
                concrete: Some(concrete),
            }),
            build_lookup_hashing_declaration_payload(Default::default()),
        );
        looked_up.declaration
    };
    let deps = build_create_commitment_deps(
        Consumer,
        CreateCommitmentDepsOverrides {
            admit_hashing_composition: Some(admit_hashing_composition),
            lookup_hashing_declaration: Some(lookup_hashing_declaration),
        },
    );
    let params = build_create_commitment_params(CreateCommitmentParamsOverrides {
        concrete: Some(concrete),
        identifier: Some(declaration.identifier),
    });
    let payload = build_create_commitment_payload(Default::default());

    // Act
    let success =
        create_commitment(&deps, params, payload).expect("the mock selection is admitted");

    // Assert
    assert_eq!(
        success.output.err(),
        Some(CommitErrorReturn::Mock(
            MockICommitmentAdapterErrorReturn::ContentLengthUnrepresentable {
                length: REFERENCE_CONTENT_LENGTH,
            }
        ))
    );
}

/// Contract: the own entry
///   `every_concrete_returns_its_own_error_when_an_offset_cannot_be_represented`:
///   `create_commitment` is called with the mock selection under the
///   failure mode `OffsetUnrepresentable` and a consumer whose
///   `consume_commitment` calls `commit` on the adapter it receives with
///   the reference content unkeyed → `Ok(CreateCommitmentSuccessReturn { output })`
///   whose `output` observes
///   `Err(CommitErrorReturn::Mock(MockICommitmentAdapterErrorReturn::OffsetUnrepresentable { offset }))`
///   with `offset` equal to `REFERENCE_CONTENT_LENGTH` as a `u64`, the
///   content's length, since `commit`'s payload carries no chunk index.
/// Arrange: as the block above, with the failure mode
///   `MockICommitmentAdapterFailureMode::OffsetUnrepresentable`.
/// Act:     `create_commitment(&deps, params, payload)` over
///   `build_create_commitment_deps` holding the `Consumer`, the real
///   `admit_hashing_composition` and `lookup_hashing_declaration`, those
///   params, and `build_create_commitment_payload`.
/// Assert:  the success arm is bound through `expect`; the `output`'s
///   `err()` equals
///   `Some(CommitErrorReturn::Mock(MockICommitmentAdapterErrorReturn::OffsetUnrepresentable { offset }))`
///   by `assert_eq!`, with `offset` `REFERENCE_CONTENT_LENGTH` converted
///   through `u64::try_from` bound by `expect`.
/// Boundary: the crate-internal path `create_commitment` through the
///   mock concrete's `try_new` with the configured failure mode, then
///   `ICommitmentConsumer::consume_commitment` calling `commit` on the
///   adapter it receives.
/// Mocked:  nothing.
#[test]
fn every_concrete_returns_its_own_error_when_an_offset_cannot_be_represented() {
    struct Consumer;

    impl ICommitmentConsumer for Consumer {
        type Output = CommitReturn;

        fn consume_commitment<A: ICommitmentAdapter>(
            &self,
            _params: ConsumeCommitmentParams,
            payload: ConsumeCommitmentPayload<A>,
        ) -> Self::Output {
            let content = reference_content(REFERENCE_CONTENT_LENGTH);
            payload.adapter.commit(
                build_commit_params(Default::default()),
                build_commit_payload(&content, CommitmentKey::Unkeyed, Default::default()),
            )
        }
    }

    // Arrange
    let concrete =
        CommitmentConcrete::Mock(MockICommitmentAdapterFailureMode::OffsetUnrepresentable);
    let declaration = {
        let Ok(looked_up) = lookup_hashing_declaration(
            &build_lookup_hashing_declaration_deps(Default::default()),
            build_lookup_hashing_declaration_params(LookupHashingDeclarationParamsOverrides {
                concrete: Some(concrete),
            }),
            build_lookup_hashing_declaration_payload(Default::default()),
        );
        looked_up.declaration
    };
    let deps = build_create_commitment_deps(
        Consumer,
        CreateCommitmentDepsOverrides {
            admit_hashing_composition: Some(admit_hashing_composition),
            lookup_hashing_declaration: Some(lookup_hashing_declaration),
        },
    );
    let params = build_create_commitment_params(CreateCommitmentParamsOverrides {
        concrete: Some(concrete),
        identifier: Some(declaration.identifier),
    });
    let payload = build_create_commitment_payload(Default::default());

    // Act
    let success =
        create_commitment(&deps, params, payload).expect("the mock selection is admitted");

    // Assert
    let offset =
        u64::try_from(REFERENCE_CONTENT_LENGTH).expect("the reference content length fits a u64");
    assert_eq!(
        success.output.err(),
        Some(CommitErrorReturn::Mock(
            MockICommitmentAdapterErrorReturn::OffsetUnrepresentable { offset }
        ))
    );
}

/// Contract: the absorbed entry `hashing/blake3_bao`
///   `commit_under_a_key_leaves_the_borrowed_key_intact`, in this node's
///   terms: `commit` is called inside `consume_commitment` on the
///   adapter it receives with `CommitmentKey::Keyed` borrowing a living
///   `Secret<Vec<u8>>` → after the call the secret's `expose()` equals
///   the bytes it held before the call.
/// Arrange: `build_secret` with the value override
///   `reference_key_bytes()`, whose bytes are not all zero, so a
///   zeroized secret differs from an intact one; for each case,
///   `build_commit_payload` over the reference content with
///   `CommitmentKey::Keyed` borrowing that secret; the block's
///   `Consumer<'a>` holding `key: &'a Secret<Vec<u8>>`, whose
///   `consume_commitment` calls `adapter.commit` on the adapter it
///   receives with the keyed payload and returns the `CommitReturn` it
///   observes as its `Output`; per case, deps carrying that consumer.
/// Act:     `create_commitment(&deps, params, payload)` for the case.
/// Assert:  the success arm is bound through `expect`; the
///   `CommitSuccessReturn` the `output` observes is bound through
///   `expect`; the secret's `expose()` equals `reference_key_bytes()`
///   by `assert_eq!` after the call.
/// Boundary: the crate-internal path `create_commitment` through the
///   selected concrete's `try_new`, then
///   `ICommitmentConsumer::consume_commitment` calling keyed `commit`
///   on the adapter it receives, with `Secret::expose` reading the key.
/// Mocked:  nothing; `blake3`, `bao`, `zeroize`, and `domain` run for
///   real.
#[test]
fn every_concrete_leaves_the_borrowed_key_intact() {
    struct Consumer<'a> {
        key: &'a Secret<Vec<u8>>,
    }

    impl ICommitmentConsumer for Consumer<'_> {
        type Output = CommitReturn;

        fn consume_commitment<A: ICommitmentAdapter>(
            &self,
            _params: ConsumeCommitmentParams,
            payload: ConsumeCommitmentPayload<A>,
        ) -> Self::Output {
            let content = reference_content(REFERENCE_CONTENT_LENGTH);
            payload.adapter.commit(
                build_commit_params(Default::default()),
                build_commit_payload(&content, CommitmentKey::Keyed(self.key), Default::default()),
            )
        }
    }

    // Arrange
    let secret = build_secret(reference_key_bytes(), SecretConstructorParamsOverrides);
    let mock_concrete = CommitmentConcrete::Mock(MockICommitmentAdapterFailureMode::NoFailure);
    for concrete in COMMITMENT_CONCRETES
        .iter()
        .map(|member| build_commitment_concrete(*member, CommitmentConcreteOverrides))
        .chain(core::iter::once(mock_concrete))
    {
        let declaration = {
            let Ok(looked_up) = lookup_hashing_declaration(
                &build_lookup_hashing_declaration_deps(Default::default()),
                build_lookup_hashing_declaration_params(LookupHashingDeclarationParamsOverrides {
                    concrete: Some(concrete),
                }),
                build_lookup_hashing_declaration_payload(Default::default()),
            );
            looked_up.declaration
        };
        let deps = build_create_commitment_deps(
            Consumer { key: &secret },
            CreateCommitmentDepsOverrides {
                admit_hashing_composition: Some(admit_hashing_composition),
                lookup_hashing_declaration: Some(lookup_hashing_declaration),
            },
        );
        let params = build_create_commitment_params(CreateCommitmentParamsOverrides {
            concrete: Some(concrete),
            identifier: Some(declaration.identifier),
        });
        let payload = build_create_commitment_payload(Default::default());

        // Act
        let success = create_commitment(&deps, params, payload)
            .expect("an admitted selection returns the consumer's output");

        // Assert
        let _committed = success.output.expect("the keyed commit is admitted");
        assert_eq!(secret.expose(), &reference_key_bytes());
    }
}

/// Contract: the absorbed entry `hashing/blake3_bao`
///   `committed_outboard_verifies_its_content`, in this node's terms:
///   `create_commitment` calls the consumer's `consume_commitment` with
///   the adapter the case selects →
///   `Ok(CreateCommitmentSuccessReturn { output })` whose `output` is a
///   pair of `VerifyReturn`s the consumer observes on that adapter: the
///   intact content with the committed root and outboard →
///   `Ok(VerifySuccessReturn)`, and the same root and outboard over the
///   content with the byte at offset `2 × L` inverted →
///   `Err(VerifyErrorReturn::ChunkMismatch { chunk_index: 2 })`.
/// Arrange: the block's `Consumer`, whose `consume_commitment` commits
///   the reference content unkeyed on the adapter it receives, the
///   success arm bound through `expect`, then calls `adapter.verify`
///   over the intact content and over a copy with the byte at offset
///   `2 × L` inverted — `L` the `chunk_length` of the declaration the
///   payload hands it, so the changed chunk is chunk 2 under each
///   concrete's own chunk length — each with the committed root and
///   outboard and `CommitmentKey::Unkeyed`, returning the pair of
///   results as its `Output`; for each case, `create_commitment` over
///   `build_create_commitment_deps` holding the `Consumer`, the real
///   `admit_hashing_composition` and `lookup_hashing_declaration`, the
///   case's params, and `build_create_commitment_payload`.
/// Act:     `create_commitment(&deps, params, payload)` for the case.
/// Assert:  the success arm is bound through `expect`; the first
///   `VerifyReturn` the `output` observes matches
///   `Ok(VerifySuccessReturn)` by `assert!(matches!(…))`; the second
///   equals `Err(VerifyErrorReturn::ChunkMismatch { chunk_index: 2 })`
///   by `assert_eq!`.
/// Boundary: the crate-internal path `create_commitment`, then
///   `ICommitmentConsumer::consume_commitment` calling `commit` and
///   `verify` on the adapter it receives.
/// Mocked:  nothing.
#[test]
fn every_concrete_verifies_the_content_it_committed() {
    struct Consumer;

    impl ICommitmentConsumer for Consumer {
        type Output = (VerifyReturn, VerifyReturn);

        fn consume_commitment<A: ICommitmentAdapter>(
            &self,
            _params: ConsumeCommitmentParams,
            payload: ConsumeCommitmentPayload<A>,
        ) -> Self::Output {
            let content = reference_content(REFERENCE_CONTENT_LENGTH);
            let committed = payload
                .adapter
                .commit(
                    build_commit_params(Default::default()),
                    build_commit_payload(&content, CommitmentKey::Unkeyed, Default::default()),
                )
                .expect("the unkeyed commit is admitted");
            let chunk_length = usize::try_from(payload.declaration.chunk_length)
                .expect("a chunk length fits a usize");
            let mut altered = content.clone();
            altered[2 * chunk_length] ^= 0xff;
            let intact = payload.adapter.verify(
                build_verify_params(Default::default()),
                build_verify_payload(
                    &content,
                    &committed.outboard,
                    &committed.root,
                    CommitmentKey::Unkeyed,
                    Default::default(),
                ),
            );
            let changed = payload.adapter.verify(
                build_verify_params(Default::default()),
                build_verify_payload(
                    &altered,
                    &committed.outboard,
                    &committed.root,
                    CommitmentKey::Unkeyed,
                    Default::default(),
                ),
            );
            (intact, changed)
        }
    }

    // Arrange
    let mock_concrete = CommitmentConcrete::Mock(MockICommitmentAdapterFailureMode::NoFailure);
    for concrete in COMMITMENT_CONCRETES
        .iter()
        .map(|member| build_commitment_concrete(*member, CommitmentConcreteOverrides))
        .chain(core::iter::once(mock_concrete))
    {
        let declaration = {
            let Ok(looked_up) = lookup_hashing_declaration(
                &build_lookup_hashing_declaration_deps(Default::default()),
                build_lookup_hashing_declaration_params(LookupHashingDeclarationParamsOverrides {
                    concrete: Some(concrete),
                }),
                build_lookup_hashing_declaration_payload(Default::default()),
            );
            looked_up.declaration
        };
        let deps = build_create_commitment_deps(
            Consumer,
            CreateCommitmentDepsOverrides {
                admit_hashing_composition: Some(admit_hashing_composition),
                lookup_hashing_declaration: Some(lookup_hashing_declaration),
            },
        );
        let params = build_create_commitment_params(CreateCommitmentParamsOverrides {
            concrete: Some(concrete),
            identifier: Some(declaration.identifier),
        });
        let payload = build_create_commitment_payload(Default::default());

        // Act
        let success = create_commitment(&deps, params, payload)
            .expect("an admitted selection returns the consumer's output");

        // Assert
        assert!(matches!(success.output.0, Ok(VerifySuccessReturn)));
        assert_eq!(
            success.output.1.err(),
            Some(VerifyErrorReturn::ChunkMismatch { chunk_index: 2 }),
        );
    }
}

/// Contract: the absorbed entry `hashing/blake3_bao`
///   `extracted_paths_verify_every_chunk`, in this node's terms:
///   `create_commitment` calls the consumer's `consume_commitment` with
///   the adapter the case selects →
///   `Ok(CreateCommitmentSuccessReturn { output })` whose `output` is
///   the `Vec<VerifyChunkReturn>` the consumer observes for every index
///   below the reference content's chunk count — each
///   `Ok(VerifyChunkSuccessReturn)` — and the `VerifyChunkReturn` it
///   observes for chunk 1's path with chunk 0's bytes at index 1 →
///   `Err(VerifyChunkErrorReturn::RootMismatch)`.
/// Arrange: the block's `Consumer`, whose `consume_commitment` commits
///   the reference content unkeyed on the adapter it receives, the
///   success arm bound through `expect`, then for each index below the
///   chunk count of the declaration the payload hands it calls
///   `extract_path` over the committed outboard with that index and
///   `verify_chunk` over the committed root, `build_chunk` of that
///   chunk's bytes, and the path, unkeyed, at the reference content
///   length and that index; then `verify_chunk` with chunk 1's path and
///   `build_chunk` of chunk 0's bytes at index 1 — chunks 0 and 1 being
///   full chunks of different bytes under every concrete — returning
///   the indexed results and the mismatched call's result as its
///   `Output`; for each case, `create_commitment` over
///   `build_create_commitment_deps` holding the `Consumer`, the real
///   `admit_hashing_composition` and `lookup_hashing_declaration`, the
///   case's params, and `build_create_commitment_payload`.
/// Act:     `create_commitment(&deps, params, payload)` for the case.
/// Assert:  the success arm is bound through `expect`; every
///   `VerifyChunkReturn` the `output`'s vector holds matches
///   `Ok(VerifyChunkSuccessReturn)` by `assert!(matches!(…))`; the
///   mismatched call's result equals
///   `Err(VerifyChunkErrorReturn::RootMismatch)` by `assert_eq!`.
/// Boundary: the crate-internal path `create_commitment`, then
///   `ICommitmentConsumer::consume_commitment` calling `commit`,
///   `extract_path`, and `verify_chunk` on the adapter it receives.
/// Mocked:  nothing.
#[test]
fn every_concrete_verifies_every_chunk_with_its_extracted_path() {
    struct Consumer;

    impl ICommitmentConsumer for Consumer {
        type Output = (Vec<VerifyChunkReturn>, VerifyChunkReturn);

        fn consume_commitment<A: ICommitmentAdapter>(
            &self,
            _params: ConsumeCommitmentParams,
            payload: ConsumeCommitmentPayload<A>,
        ) -> Self::Output {
            let content = reference_content(REFERENCE_CONTENT_LENGTH);
            let committed = payload
                .adapter
                .commit(
                    build_commit_params(Default::default()),
                    build_commit_payload(&content, CommitmentKey::Unkeyed, Default::default()),
                )
                .expect("the unkeyed commit is admitted");
            let chunk_length = payload.declaration.chunk_length;
            let chunk_count = u64::try_from(REFERENCE_CONTENT_LENGTH)
                .expect("the reference content length fits a u64")
                .div_ceil(chunk_length);
            let mut results = Vec::new();
            for index in 0..chunk_count {
                let path = payload
                    .adapter
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
                let start =
                    usize::try_from(index * chunk_length).expect("a chunk offset fits a usize");
                let end = start
                    + usize::try_from(chunk_length)
                        .expect("a chunk length fits a usize")
                        .min(REFERENCE_CONTENT_LENGTH - start);
                let chunk = build_chunk(ChunkOverrides {
                    bytes: Some(content[start..end].to_vec()),
                });
                results.push(payload.adapter.verify_chunk(
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
            let path_one = payload
                .adapter
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
            let mismatched = payload.adapter.verify_chunk(
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
            (results, mismatched)
        }
    }

    // Arrange
    let mock_concrete = CommitmentConcrete::Mock(MockICommitmentAdapterFailureMode::NoFailure);
    for concrete in COMMITMENT_CONCRETES
        .iter()
        .map(|member| build_commitment_concrete(*member, CommitmentConcreteOverrides))
        .chain(core::iter::once(mock_concrete))
    {
        let declaration = {
            let Ok(looked_up) = lookup_hashing_declaration(
                &build_lookup_hashing_declaration_deps(Default::default()),
                build_lookup_hashing_declaration_params(LookupHashingDeclarationParamsOverrides {
                    concrete: Some(concrete),
                }),
                build_lookup_hashing_declaration_payload(Default::default()),
            );
            looked_up.declaration
        };
        let deps = build_create_commitment_deps(
            Consumer,
            CreateCommitmentDepsOverrides {
                admit_hashing_composition: Some(admit_hashing_composition),
                lookup_hashing_declaration: Some(lookup_hashing_declaration),
            },
        );
        let params = build_create_commitment_params(CreateCommitmentParamsOverrides {
            concrete: Some(concrete),
            identifier: Some(declaration.identifier),
        });
        let payload = build_create_commitment_payload(Default::default());

        // Act
        let success = create_commitment(&deps, params, payload)
            .expect("an admitted selection returns the consumer's output");

        // Assert
        for result in success.output.0 {
            assert!(matches!(result, Ok(VerifyChunkSuccessReturn)));
        }
        assert_eq!(
            success.output.1.err(),
            Some(VerifyChunkErrorReturn::RootMismatch),
        );
    }
}

/// Contract: the absorbed entry `hashing/blake3_bao`
///   `a_challenge_response_verifies_against_the_root`, in this node's
///   terms: `create_commitment` calls the consumer's
///   `consume_commitment` with the adapter the case selects →
///   `Ok(CreateCommitmentSuccessReturn { output })` whose `output` is a
///   pair of `VerifyChunkReturn`s the consumer observes on that
///   adapter: `respond` answering the chunk index `challenge` chose
///   from fixed uniform bytes over the committed content and outboard,
///   checked by `verify_chunk` against the committed root →
///   `Ok(VerifyChunkSuccessReturn)`; and the response computed over the
///   outboard with one byte inverted in each of the two child values
///   the root's parent node records, checked the same way →
///   `Err(VerifyChunkErrorReturn::RootMismatch)`.
/// Arrange: the block's `Consumer`, whose `consume_commitment` commits
///   the reference content unkeyed on the adapter it receives, the
///   success arm bound through `expect`; builds a copy of the committed
///   outboard through `build_outboard` with the bytes at offsets `8`
///   and `40`, the first byte of each child value of the root's parent
///   node, inverted; then calls `challenge` with the uniform override
///   the little-endian bytes of 6, `respond` at the chosen index over
///   the intact and the altered outboard, and `verify_chunk` of each
///   response against the committed root, unkeyed, at that index and
///   the reference content length, returning the pair of results as
///   its `Output`; for each case, `create_commitment` over
///   `build_create_commitment_deps` holding the `Consumer`, the real
///   `admit_hashing_composition` and `lookup_hashing_declaration`, the
///   case's params, and `build_create_commitment_payload`.
/// Act:     `create_commitment(&deps, params, payload)` for the case.
/// Assert:  the success arm is bound through `expect`; the first
///   `VerifyChunkReturn` the `output` observes matches
///   `Ok(VerifyChunkSuccessReturn)` by `assert!(matches!(…))`; the
///   second equals `Err(VerifyChunkErrorReturn::RootMismatch)` by
///   `assert_eq!`.
/// Boundary: the crate-internal path `create_commitment`, then
///   `ICommitmentConsumer::consume_commitment` calling `commit`,
///   `challenge`, `respond`, and `verify_chunk` on the adapter it
///   receives.
/// Mocked:  nothing.
#[test]
fn every_concrete_answers_a_challenge_with_a_response_that_verifies() {
    struct Consumer;

    impl ICommitmentConsumer for Consumer {
        type Output = (VerifyChunkReturn, VerifyChunkReturn);

        fn consume_commitment<A: ICommitmentAdapter>(
            &self,
            _params: ConsumeCommitmentParams,
            payload: ConsumeCommitmentPayload<A>,
        ) -> Self::Output {
            let content = reference_content(REFERENCE_CONTENT_LENGTH);
            let committed = payload
                .adapter
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
            let index = payload
                .adapter
                .challenge(
                    build_challenge_params(Default::default()),
                    build_challenge_payload(ChallengePayloadOverrides {
                        uniform: Some(6u64.to_le_bytes()),
                        ..Default::default()
                    }),
                )
                .expect("the challenge always succeeds")
                .chunk_index;
            let intact_response = payload
                .adapter
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
            let intact = payload.adapter.verify_chunk(
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
            let altered_response = payload
                .adapter
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
            let altered_result = payload.adapter.verify_chunk(
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
            (intact, altered_result)
        }
    }

    // Arrange
    let mock_concrete = CommitmentConcrete::Mock(MockICommitmentAdapterFailureMode::NoFailure);
    for concrete in COMMITMENT_CONCRETES
        .iter()
        .map(|member| build_commitment_concrete(*member, CommitmentConcreteOverrides))
        .chain(core::iter::once(mock_concrete))
    {
        let declaration = {
            let Ok(looked_up) = lookup_hashing_declaration(
                &build_lookup_hashing_declaration_deps(Default::default()),
                build_lookup_hashing_declaration_params(LookupHashingDeclarationParamsOverrides {
                    concrete: Some(concrete),
                }),
                build_lookup_hashing_declaration_payload(Default::default()),
            );
            looked_up.declaration
        };
        let deps = build_create_commitment_deps(
            Consumer,
            CreateCommitmentDepsOverrides {
                admit_hashing_composition: Some(admit_hashing_composition),
                lookup_hashing_declaration: Some(lookup_hashing_declaration),
            },
        );
        let params = build_create_commitment_params(CreateCommitmentParamsOverrides {
            concrete: Some(concrete),
            identifier: Some(declaration.identifier),
        });
        let payload = build_create_commitment_payload(Default::default());

        // Act
        let success = create_commitment(&deps, params, payload)
            .expect("an admitted selection returns the consumer's output");

        // Assert
        assert!(matches!(success.output.0, Ok(VerifyChunkSuccessReturn)));
        assert_eq!(
            success.output.1.err(),
            Some(VerifyChunkErrorReturn::RootMismatch),
        );
    }
}

/// Contract: the absorbed entry `hashing/blake3_bao`
///   `keyed_paths_verify_only_under_their_key`, in this node's terms:
///   inside `consume_commitment`, a path `extract_path` returns from a
///   keyed commitment is checked with `verify_chunk` under the same key
///   and unkeyed → `Ok(VerifyChunkSuccessReturn)` under the key and
///   `Err(VerifyChunkErrorReturn::RootMismatch)` unkeyed.
/// Arrange: `build_secret` with the value override
///   `reference_key_bytes()`; for each case, `build_commit_payload`
///   over the reference content with `CommitmentKey::Keyed` borrowing
///   that secret; the block's `Consumer<'a>` holding
///   `key: &'a Secret<Vec<u8>>`, whose `consume_commitment` calls
///   `commit` with the keyed payload on the adapter it receives, the
///   success arm bound through `expect`; then `extract_path` for index
///   2 and `verify_chunk` at index 2 over the committed root under
///   `CommitmentKey::Keyed` borrowing the secret and under
///   `CommitmentKey::Unkeyed` — the chunk bounds from the
///   `chunk_length` of the declaration the payload hands it — and
///   returns the observed results as its `Output`; per case, deps
///   carrying that consumer.
/// Act:     `create_commitment(&deps, params, payload)` for the case.
/// Assert:  the success arm is bound through `expect`; under the key
///   the result the `output` holds matches
///   `Ok(VerifyChunkSuccessReturn)` by `assert!(matches!(…))`; unkeyed
///   the result equals `Err(VerifyChunkErrorReturn::RootMismatch)` by
///   `assert_eq!`.
/// Boundary: the crate-internal path `create_commitment`, then
///   `ICommitmentConsumer::consume_commitment` calling keyed `commit`,
///   `extract_path`, and `verify_chunk` on the adapter it receives.
/// Mocked:  nothing; `blake3`, `bao`, `zeroize`, and `domain` run for
///   real.
#[test]
fn every_concrete_verifies_a_keyed_path_only_under_its_key() {
    struct Consumer<'a> {
        key: &'a Secret<Vec<u8>>,
    }

    impl ICommitmentConsumer for Consumer<'_> {
        type Output = (VerifyChunkReturn, VerifyChunkReturn);

        fn consume_commitment<A: ICommitmentAdapter>(
            &self,
            _params: ConsumeCommitmentParams,
            payload: ConsumeCommitmentPayload<A>,
        ) -> Self::Output {
            let content = reference_content(REFERENCE_CONTENT_LENGTH);
            let committed = payload
                .adapter
                .commit(
                    build_commit_params(Default::default()),
                    build_commit_payload(
                        &content,
                        CommitmentKey::Keyed(self.key),
                        Default::default(),
                    ),
                )
                .expect("the keyed commit is admitted");
            let path = payload
                .adapter
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
            let chunk_length = payload.declaration.chunk_length;
            let start = usize::try_from(2 * chunk_length).expect("a chunk offset fits a usize");
            let end = start
                + usize::try_from(chunk_length)
                    .expect("a chunk length fits a usize")
                    .min(REFERENCE_CONTENT_LENGTH - start);
            let chunk = build_chunk(ChunkOverrides {
                bytes: Some(content[start..end].to_vec()),
            });
            let keyed = payload.adapter.verify_chunk(
                build_verify_chunk_params(Default::default()),
                build_verify_chunk_payload(
                    &committed.root,
                    &chunk,
                    &path,
                    CommitmentKey::Keyed(self.key),
                    VerifyChunkPayloadOverrides {
                        chunk_index: Some(2),
                        ..Default::default()
                    },
                ),
            );
            let unkeyed = payload.adapter.verify_chunk(
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
            (keyed, unkeyed)
        }
    }

    // Arrange
    let secret = build_secret(reference_key_bytes(), SecretConstructorParamsOverrides);
    let mock_concrete = CommitmentConcrete::Mock(MockICommitmentAdapterFailureMode::NoFailure);
    for concrete in COMMITMENT_CONCRETES
        .iter()
        .map(|member| build_commitment_concrete(*member, CommitmentConcreteOverrides))
        .chain(core::iter::once(mock_concrete))
    {
        let declaration = {
            let Ok(looked_up) = lookup_hashing_declaration(
                &build_lookup_hashing_declaration_deps(Default::default()),
                build_lookup_hashing_declaration_params(LookupHashingDeclarationParamsOverrides {
                    concrete: Some(concrete),
                }),
                build_lookup_hashing_declaration_payload(Default::default()),
            );
            looked_up.declaration
        };
        let deps = build_create_commitment_deps(
            Consumer { key: &secret },
            CreateCommitmentDepsOverrides {
                admit_hashing_composition: Some(admit_hashing_composition),
                lookup_hashing_declaration: Some(lookup_hashing_declaration),
            },
        );
        let params = build_create_commitment_params(CreateCommitmentParamsOverrides {
            concrete: Some(concrete),
            identifier: Some(declaration.identifier),
        });
        let payload = build_create_commitment_payload(Default::default());

        // Act
        let success = create_commitment(&deps, params, payload)
            .expect("an admitted selection returns the consumer's output");

        // Assert
        assert!(matches!(success.output.0, Ok(VerifyChunkSuccessReturn)));
        assert_eq!(
            success.output.1.err(),
            Some(VerifyChunkErrorReturn::RootMismatch),
        );
    }
}
