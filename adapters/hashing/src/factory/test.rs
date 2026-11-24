#![allow(clippy::expect_used)]

use super::provides::{
    CommitErrorReturn, CommitmentConcrete, CommitmentKey, CommitmentSchemeIdentifier,
    CreateCommitmentDepsOverrides, CreateCommitmentErrorReturn, CreateCommitmentParamsOverrides,
    ICommitmentAdapter, MockICommitmentAdapterErrorReturn, MockICommitmentAdapterFailureMode,
    REFERENCE_CONTENT_LENGTH, Root, RootConstructorParamsOverrides, RootTryNewErrorReturn,
    build_commit_params, build_commit_payload, build_create_commitment_deps,
    build_create_commitment_params, build_create_commitment_payload, build_root_constructor_params,
    create_commitment, reference_content,
};
use crate::admit_hashing_composition::provides::{
    AdmitHashingCompositionDeps, AdmitHashingCompositionErrorReturn, AdmitHashingCompositionParams,
    AdmitHashingCompositionPayload, AdmitHashingCompositionReturn,
};

/// Contract: `Root::try_new`, admitted: some byte of `params.bytes` is nonzero →
///   `Ok(Root)` holding those bytes.
/// Arrange: `build_root_constructor_params` with the bytes override 31 zero
///   bytes followed by `0x01`, so only the last byte separates it from the
///   refused value.
/// Act:     `Root::try_new`.
/// Assert:  the success arm is bound through `expect`; its `as_bytes()` equals
///   the literal of 31 zero bytes followed by `0x01` by `assert_eq!`.
#[test]
fn root_try_new_holds_nonzero_bytes() {
    // Arrange
    let mut bytes = [0u8; 32];
    bytes[31] = 0x01;
    let params =
        build_root_constructor_params(RootConstructorParamsOverrides { bytes: Some(bytes) });

    // Act
    let result = Root::try_new(params);

    // Assert
    let root = result.expect("a byte sequence with a nonzero byte is admitted");
    assert_eq!(
        root.as_bytes(),
        &[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 1,
        ],
    );
}

/// Contract: `Root::try_new`, refused: every byte of `params.bytes` is zero →
///   `Err(RootTryNewErrorReturn::AllZero)`.
/// Arrange: `build_root_constructor_params` with the bytes override 32 zero
///   bytes, the one byte sequence the constructor refuses.
/// Act:     `Root::try_new`.
/// Assert:  `result` equals `Err(RootTryNewErrorReturn::AllZero)` by
///   `assert_eq!`.
#[test]
fn root_try_new_refuses_all_zero_bytes() {
    // Arrange
    let params = build_root_constructor_params(RootConstructorParamsOverrides {
        bytes: Some([0u8; 32]),
    });

    // Act
    let result = Root::try_new(params);

    // Assert
    assert_eq!(result, Err(RootTryNewErrorReturn::AllZero));
}

/// Contract: `create_commitment`, composition admitted →
///   `Ok(CreateCommitmentSuccessReturn { adapter })` whose adapter is the
///   concrete the selection names, answering under the failure mode the
///   selection carries.
/// Collaborators: the family's mock concrete, reached through
///   `create_commitment` by the `concrete` override;
///   `build_create_commitment_deps` at its defaults, whose function mocks
///   admit; `build_create_commitment_payload` at its default; no test names
///   a concrete.
/// Arrange: `build_create_commitment_params` with the `concrete` override
///   `CommitmentConcrete::Mock(MockICommitmentAdapterFailureMode::ContentLengthUnrepresentable)`,
///   a mode other than the `NoFailure` the default holds, so a factory
///   that ignores the selection's mode fails; `build_commit_payload` over
///   `reference_content(REFERENCE_CONTENT_LENGTH)` with
///   `CommitmentKey::Unkeyed`.
/// Act:     `create_commitment(&deps, params, payload)`, then
///   `adapter.commit(build_commit_params(Default::default()), commit_payload)`.
/// Assert:  the success arm is bound through `expect`; the commit result's
///   `err()` equals
///   `Some(CommitErrorReturn::Mock(MockICommitmentAdapterErrorReturn::ContentLengthUnrepresentable { length: REFERENCE_CONTENT_LENGTH }))`
///   by `assert_eq!`, the expectation a literal and not read from the
///   arrangement.
#[test]
fn create_commitment_returns_the_adapter_of_the_selected_concrete() {
    // Arrange
    let deps = build_create_commitment_deps(Default::default());
    let params = build_create_commitment_params(CreateCommitmentParamsOverrides {
        concrete: Some(CommitmentConcrete::Mock(
            MockICommitmentAdapterFailureMode::ContentLengthUnrepresentable,
        )),
        ..Default::default()
    });
    let payload = build_create_commitment_payload(Default::default());
    let content = reference_content(REFERENCE_CONTENT_LENGTH);
    let commit_payload = build_commit_payload(&content, CommitmentKey::Unkeyed, Default::default());

    // Act
    let success =
        create_commitment(&deps, params, payload).expect("the function mocks admit the selection");
    let adapter: &dyn ICommitmentAdapter = &*success.adapter;
    let commit_result = adapter.commit(build_commit_params(Default::default()), commit_payload);

    // Assert
    assert_eq!(
        commit_result.err(),
        Some(CommitErrorReturn::Mock(
            MockICommitmentAdapterErrorReturn::ContentLengthUnrepresentable {
                length: REFERENCE_CONTENT_LENGTH,
            },
        )),
    );
}

/// Contract: `create_commitment`, composition refused: the admission
///   returns `Err(error)` →
///   `Err(CreateCommitmentErrorReturn::Composition(error))`, the error
///   unchanged.
/// Collaborators: the family's mock concrete, reached through
///   `create_commitment` by `build_create_commitment_params` at its
///   defaults; `build_create_commitment_deps` with the
///   `admit_hashing_composition` override a function the block declares of
///   the production type `AdmitHashingCompositionFn`;
///   `build_create_commitment_payload` at its default; no test names a
///   concrete.
/// Arrange: the declared function returns
///   `Err(AdmitHashingCompositionErrorReturn::Incompatible { concrete: CommitmentConcrete::Mock(MockICommitmentAdapterFailureMode::OffsetUnrepresentable), declared: CommitmentSchemeIdentifier::Blake3BaoV1, required: CommitmentSchemeIdentifier::Blake3BaoV1 })`,
///   a `concrete` other than the default selection's, so a factory that
///   builds its own refusal from `params.concrete` fails.
/// Act:     `create_commitment(&deps, params, payload)`.
/// Assert:  `result.err()` equals
///   `Some(CreateCommitmentErrorReturn::Composition(AdmitHashingCompositionErrorReturn::Incompatible { concrete: CommitmentConcrete::Mock(MockICommitmentAdapterFailureMode::OffsetUnrepresentable), declared: CommitmentSchemeIdentifier::Blake3BaoV1, required: CommitmentSchemeIdentifier::Blake3BaoV1 }))`
///   by `assert_eq!`.
#[test]
fn create_commitment_returns_the_composition_refusal_unchanged() {
    fn refuse(
        _deps: &AdmitHashingCompositionDeps,
        _params: AdmitHashingCompositionParams,
        _payload: AdmitHashingCompositionPayload,
    ) -> AdmitHashingCompositionReturn {
        Err(AdmitHashingCompositionErrorReturn::Incompatible {
            concrete: CommitmentConcrete::Mock(
                MockICommitmentAdapterFailureMode::OffsetUnrepresentable,
            ),
            declared: CommitmentSchemeIdentifier::Blake3BaoV1,
            required: CommitmentSchemeIdentifier::Blake3BaoV1,
        })
    }

    // Arrange
    let deps = build_create_commitment_deps(CreateCommitmentDepsOverrides {
        admit_hashing_composition: Some(refuse),
        ..Default::default()
    });
    let params = build_create_commitment_params(Default::default());
    let payload = build_create_commitment_payload(Default::default());

    // Act
    let result = create_commitment(&deps, params, payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(CreateCommitmentErrorReturn::Composition(
            AdmitHashingCompositionErrorReturn::Incompatible {
                concrete: CommitmentConcrete::Mock(
                    MockICommitmentAdapterFailureMode::OffsetUnrepresentable,
                ),
                declared: CommitmentSchemeIdentifier::Blake3BaoV1,
                required: CommitmentSchemeIdentifier::Blake3BaoV1,
            },
        )),
    );
}
