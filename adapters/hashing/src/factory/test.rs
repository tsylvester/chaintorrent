#![allow(clippy::expect_used)]

use super::provides::{
    CommitErrorReturn, CommitReturn, CommitmentConcrete, CommitmentDeclaration,
    CommitmentDeclarationOverrides, CommitmentKey, CommitmentSchemeIdentifier,
    ConsumeCommitmentParams, ConsumeCommitmentPayload, CreateCommitmentDepsOverrides,
    CreateCommitmentErrorReturn, CreateCommitmentParamsOverrides, ICommitmentAdapter,
    ICommitmentConsumer, MockICommitmentAdapterErrorReturn, MockICommitmentAdapterFailureMode,
    MockICommitmentConsumer, REFERENCE_CONTENT_LENGTH, Root, RootConstructorParamsOverrides,
    RootTryNewErrorReturn, build_commit_params, build_commit_payload, build_commitment_declaration,
    build_create_commitment_deps, build_create_commitment_params, build_create_commitment_payload,
    build_root_constructor_params, create_commitment, reference_content,
};
use crate::admit_hashing_composition::provides::{
    AdmitHashingCompositionDeps, AdmitHashingCompositionErrorReturn, AdmitHashingCompositionParams,
    AdmitHashingCompositionPayload, AdmitHashingCompositionReturn,
};
use crate::lookup_hashing_declaration::provides::{
    LookupHashingDeclarationFn, LookupHashingDeclarationParams,
    LookupHashingDeclarationSuccessReturn,
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
///   `Ok(CreateCommitmentSuccessReturn { output })` relaying the consumer's
///   output, the factory handing the constructed adapter and the
///   declaration its `lookup_hashing_declaration` call returned for
///   `params.concrete` to the consumer in `ConsumeCommitmentPayload`.
/// Collaborators: the family's mock concrete, reached through
///   `create_commitment` by the `concrete` override;
///   `MockICommitmentConsumer`, whose `Output` is the
///   `CommitmentDeclaration` it returns from `payload.declaration`, and a
///   block-local consumer whose `consume_commitment` calls `adapter.commit`
///   on the adapter it receives and returns the `CommitReturn` as its
///   `Output`, each placed in `deps` by `build_create_commitment_deps`; a
///   declaration lookup the block declares, of the production type
///   `LookupHashingDeclarationFn`; `build_create_commitment_payload` at its
///   default; no test names a concrete.
/// Arrange: `build_create_commitment_params` with the `concrete` override
///   `CommitmentConcrete::Mock(MockICommitmentAdapterFailureMode::ContentLengthUnrepresentable)`,
///   a mode other than the `NoFailure` the default holds, so a factory
///   that ignores the selection's mode fails; the declared lookup returns
///   `Ok(LookupHashingDeclarationSuccessReturn { declaration })` whose
///   `declaration` `build_commitment_declaration` builds with `concrete`
///   overridden to the `concrete` the call's `LookupHashingDeclarationParams`
///   carries, so the assert proves the factory called the lookup with
///   `params.concrete` and handed its return to the consumer;
///   `build_commit_payload` over `reference_content(REFERENCE_CONTENT_LENGTH)`
///   with `CommitmentKey::Unkeyed`.
/// Act:     `create_commitment(&deps, params, payload)` once with
///   `MockICommitmentConsumer` in `deps` and once with the block-local
///   consumer.
/// Assert:  each success arm is bound through `expect`; the first call's
///   `output` is the declaration whose `concrete` equals `params.concrete`
///   by `assert_eq!`; the second call's `output` is the `CommitReturn` the
///   consumer observed, whose `err()` equals
///   `Some(CommitErrorReturn::Mock(MockICommitmentAdapterErrorReturn::ContentLengthUnrepresentable { length: REFERENCE_CONTENT_LENGTH }))`
///   by `assert_eq!`, the expectation a literal and not read from the
///   arrangement.
#[test]
fn create_commitment_constructs_the_selected_concrete_and_returns_the_consumers_output() {
    struct CommitConsumer;

    impl ICommitmentConsumer for CommitConsumer {
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

    let lookup: LookupHashingDeclarationFn =
        |_deps, params: LookupHashingDeclarationParams, _payload| {
            let declaration: CommitmentDeclaration =
                build_commitment_declaration(CommitmentDeclarationOverrides {
                    concrete: Some(params.concrete),
                    ..Default::default()
                });
            Ok(LookupHashingDeclarationSuccessReturn { declaration })
        };

    // Arrange
    let params = build_create_commitment_params(CreateCommitmentParamsOverrides {
        concrete: Some(CommitmentConcrete::Mock(
            MockICommitmentAdapterFailureMode::ContentLengthUnrepresentable,
        )),
        ..Default::default()
    });
    let declaration_deps = build_create_commitment_deps(
        MockICommitmentConsumer,
        CreateCommitmentDepsOverrides {
            lookup_hashing_declaration: Some(lookup),
            ..Default::default()
        },
    );
    let commit_deps = build_create_commitment_deps(
        CommitConsumer,
        CreateCommitmentDepsOverrides {
            lookup_hashing_declaration: Some(lookup),
            ..Default::default()
        },
    );

    // Act
    let declaration_success = create_commitment(
        &declaration_deps,
        params,
        build_create_commitment_payload(Default::default()),
    )
    .expect("the function mocks admit the selection");
    let commit_success = create_commitment(
        &commit_deps,
        params,
        build_create_commitment_payload(Default::default()),
    )
    .expect("the function mocks admit the selection");

    // Assert
    assert_eq!(declaration_success.output.concrete, params.concrete);
    assert_eq!(
        commit_success.output.err(),
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
///   defaults; `build_create_commitment_deps` with `MockICommitmentConsumer`
///   leading and the `admit_hashing_composition` override a function the
///   block declares of the production type `AdmitHashingCompositionFn`;
///   `build_create_commitment_payload` at its default; no test names a
///   concrete.
/// Arrange: the declared function returns
///   `Err(AdmitHashingCompositionErrorReturn::Incompatible { concrete: CommitmentConcrete::Mock(MockICommitmentAdapterFailureMode::OffsetUnrepresentable), declared: CommitmentSchemeIdentifier::Blake3BaoV1, required: CommitmentSchemeIdentifier::Blake3BaoV1 })`,
///   a `concrete` other than the default selection's, so a factory that
///   builds its own refusal from `params.concrete` fails; the lookup and
///   the consumer are unreached and uncalled on the refusal, so the block
///   proves no construction or consumer call follows `Err`.
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
    let deps = build_create_commitment_deps(
        MockICommitmentConsumer,
        CreateCommitmentDepsOverrides {
            admit_hashing_composition: Some(refuse),
            ..Default::default()
        },
    );
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
