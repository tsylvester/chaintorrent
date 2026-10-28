#![allow(clippy::expect_used)]

use super::provides::{
    CommitmentSchemeIdentifier, CreateCommitmentErrorReturn, CreateCommitmentParamsOverrides,
    ICommitmentAdapter, Root, RootConstructorParamsOverrides, RootTryNewErrorReturn,
    build_create_commitment_deps, build_create_commitment_params, build_create_commitment_payload,
    build_root_constructor_params, create_commitment,
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

/// Contract: `create_commitment`, admitted: the named concrete's declared
///   identifier is `params.identifier` →
///   `Ok(CreateCommitmentSuccessReturn { adapter })` whose adapter reports
///   the declaration of the concrete the selection names.
/// Arrange: `build_create_commitment_deps`,
///   `build_create_commitment_params`, and
///   `build_create_commitment_payload`, each with no override, so the
///   defaults select the family's mock concrete under the identifier it
///   declares; the selected concrete declares an identifier no other
///   concrete declares, so a factory that constructs another concrete
///   reports another identifier.
/// Act:     `create_commitment(&deps, params, payload)`.
/// Assert:  the success arm is bound through `expect`;
///   `adapter.declaration().identifier` equals
///   `CommitmentSchemeIdentifier::MockV1`, the identifier the family's mock
///   concrete declares, by `assert_eq!`, the expectation a literal and not
///   read from the arrangement.
#[test]
fn create_commitment_returns_the_adapter_of_the_selected_concrete() {
    // Arrange
    let deps = build_create_commitment_deps(Default::default());
    let params = build_create_commitment_params(Default::default());
    let payload = build_create_commitment_payload(Default::default());

    // Act
    let result = create_commitment(&deps, params, payload);

    // Assert
    let success = result.expect("the default selection is admitted");
    let adapter: &dyn ICommitmentAdapter = &*success.adapter;
    assert_eq!(
        adapter.declaration().identifier,
        CommitmentSchemeIdentifier::MockV1
    );
}

/// Contract: `create_commitment`, unsupported identifier: the named
///   concrete's `DECLARATION.identifier` is not `params.identifier` →
///   `Err(CreateCommitmentErrorReturn::UnsupportedCommitmentScheme)`.
/// Arrange: `build_create_commitment_deps` and
///   `build_create_commitment_payload` with no override;
///   `build_create_commitment_params` overriding `identifier` with
///   `CommitmentSchemeIdentifier::Blake3BaoV1` and keeping the default
///   selection of the mock concrete, which declares
///   `CommitmentSchemeIdentifier::MockV1`, so the declared identifier
///   differs from the required one.
/// Act:     `create_commitment(&deps, params, payload)`.
/// Assert:  `result.err()` equals
///   `Some(CreateCommitmentErrorReturn::UnsupportedCommitmentScheme)` by
///   `assert_eq!`.
#[test]
fn create_commitment_refuses_a_concrete_that_declares_another_identifier() {
    // Arrange
    let deps = build_create_commitment_deps(Default::default());
    let params = build_create_commitment_params(CreateCommitmentParamsOverrides {
        identifier: Some(CommitmentSchemeIdentifier::Blake3BaoV1),
        ..Default::default()
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
