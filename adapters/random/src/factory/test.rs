#![allow(clippy::expect_used)]

use super::provides::{
    CreateRandomSourceParamsOverrides, FillBytesErrorReturn, FillBytesParams,
    FillBytesPayloadOverrides, IRandomSourceAdapter, MockIRandomSourceAdapterFailureMode,
    RANDOM_SOURCE_KINDS, RandomSourceKind, build_create_random_source_deps,
    build_create_random_source_params, build_create_random_source_payload,
    build_fill_bytes_payload, build_random_source_kind, create_random_source,
};

/// Contract: `create_random_source` is called with `params.kind` the selection
///   `build_random_source_kind` returns for a member of `RANDOM_SOURCE_KINDS` →
///   `Ok(CreateRandomSourceSuccessReturn { adapter })` holding the real
///   concrete that member names, whose draws come from its generator and not
///   from the mock concrete's counter.
/// Collaborators: the `try_new` of the concrete each member names, which the
///   factory constructs, runs for real, and the member's generator runs as
///   the outer edge; fixtures `build_create_random_source_params`,
///   `build_create_random_source_deps`, `build_create_random_source_payload`,
///   `build_random_source_kind`, and `build_fill_bytes_payload`.
/// Arrange: the block's body run over each member of `RANDOM_SOURCE_KINDS`,
///   `build_create_random_source_params` with the kind override set to
///   `build_random_source_kind(member, Default::default())`; two sources
///   built from two calls to `create_random_source` over that selection, so
///   a factory arm that constructs the mock concrete, whose fixed counter
///   yields the same first draw from every fresh instance, fails.
/// Act:     `create_random_source` over the built deps, params, and payload
///   once for each source, then `fill_bytes(FillBytesParams,
///   build_fill_bytes_payload(Default::default()))` once on each.
/// Assert:  each success arm is extracted with `expect`; the `bytes.expose()`
///   of the first source differs from the `bytes.expose()` of the second
///   under `assert_ne!`.
#[test]
fn every_member_of_the_declared_set_is_constructed_into_a_source_drawing_from_the_real_generator() {
    for member in RANDOM_SOURCE_KINDS {
        // Arrange
        let kind = build_random_source_kind(*member, Default::default());
        let first_params = build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(kind),
        });
        let second_params = build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(kind),
        });

        // Act
        let first: Box<dyn IRandomSourceAdapter> = create_random_source(
            &build_create_random_source_deps(Default::default()),
            first_params,
            build_create_random_source_payload(Default::default()),
        )
        .expect("every member of the declared set constructs its source")
        .adapter;
        let second: Box<dyn IRandomSourceAdapter> = create_random_source(
            &build_create_random_source_deps(Default::default()),
            second_params,
            build_create_random_source_payload(Default::default()),
        )
        .expect("every member of the declared set constructs its source")
        .adapter;
        let first_result = first.fill_bytes(
            FillBytesParams,
            build_fill_bytes_payload(Default::default()),
        );
        let second_result = second.fill_bytes(
            FillBytesParams,
            build_fill_bytes_payload(Default::default()),
        );

        // Assert
        let first_success = first_result.expect("the member's source fills the buffer");
        let second_success = second_result.expect("the member's source fills the buffer");
        assert_ne!(first_success.bytes.expose(), second_success.bytes.expose());
    }
}
// every_member_of_the_declared_set_is_constructed_into_a_source_drawing_from_the_real_generator

/// Contract: `params.kind` is `RandomSourceKind::Mock(
///   MockIRandomSourceAdapterFailureMode::FillBytesRefused)` →
///   `Ok(CreateRandomSourceSuccessReturn { adapter })` holding the mock
///   concrete, whose `fill_bytes` returns
///   `Err(FillBytesErrorReturn::MockIRandomSourceAdapter)`.
/// Collaborators: `MockIRandomSourceAdapter::try_new`, the concrete the
///   factory constructs, runs for real; fixtures
///   `build_create_random_source_params`, `build_create_random_source_deps`,
///   `build_create_random_source_payload`, and `build_fill_bytes_payload`.
/// Arrange: `build_create_random_source_params` with the kind override set to
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)`, a mode that differs from the builder's default
///   `Succeeds`, so an arm that drops the mode, whose draw would succeed,
///   fails.
/// Act:     `create_random_source` over the built deps, params, and payload,
///   then `fill_bytes(FillBytesParams,
///   build_fill_bytes_payload(Default::default()))` on the success arm's
///   adapter.
/// Assert:  the adapter is extracted from the success arm with `expect`; the
///   `fill_bytes` result's `err()` equals
///   `Some(FillBytesErrorReturn::MockIRandomSourceAdapter)` under
///   `assert_eq!`.
#[test]
fn create_random_source_returns_the_mock_source_failing_in_the_mode_its_kind_names() {
    // Arrange
    let params = build_create_random_source_params(CreateRandomSourceParamsOverrides {
        kind: Some(RandomSourceKind::Mock(
            MockIRandomSourceAdapterFailureMode::FillBytesRefused,
        )),
    });

    // Act
    let adapter: Box<dyn IRandomSourceAdapter> = create_random_source(
        &build_create_random_source_deps(Default::default()),
        params,
        build_create_random_source_payload(Default::default()),
    )
    .expect("the mock selection constructs its source")
    .adapter;
    let result = adapter.fill_bytes(
        FillBytesParams,
        build_fill_bytes_payload(Default::default()),
    );

    // Assert
    assert_eq!(
        result.err(),
        Some(FillBytesErrorReturn::MockIRandomSourceAdapter)
    );
}
// create_random_source_returns_the_mock_source_failing_in_the_mode_its_kind_names

/// Contract: `params.kind` is `RandomSourceKind::Mock(
///   MockIRandomSourceAdapterFailureMode::Succeeds)` →
///   `Ok(CreateRandomSourceSuccessReturn { adapter })` holding the mock
///   concrete, whose `fill_bytes` returns `Ok` holding exactly
///   `payload.length` bytes.
/// Collaborators: `MockIRandomSourceAdapter::try_new`, the concrete the
///   factory constructs, runs for real; fixtures
///   `build_create_random_source_params`, `build_create_random_source_deps`,
///   `build_create_random_source_payload`, and `build_fill_bytes_payload`.
/// Arrange: `build_create_random_source_params` with the kind override set to
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::Succeeds)`,
///   and `build_fill_bytes_payload` with the length override set to 48, a
///   length that differs from the builder's default of 32, so an arm that
///   fixes the mode to `FillBytesRefused` fails and a source that returns
///   the default width fails.
/// Act:     `create_random_source` over the built deps, params, and payload,
///   then `fill_bytes(FillBytesParams, payload)` on the success arm's
///   adapter.
/// Assert:  the adapter and the `fill_bytes` success arm are each extracted
///   with `expect`; the length of `bytes.expose()` equals the literal 48
///   written in the assertion.
#[test]
fn create_random_source_returns_the_mock_source_drawing_the_bytes_requested_in_the_succeeds_mode() {
    // Arrange
    let params = build_create_random_source_params(CreateRandomSourceParamsOverrides {
        kind: Some(RandomSourceKind::Mock(
            MockIRandomSourceAdapterFailureMode::Succeeds,
        )),
    });
    let payload = build_fill_bytes_payload(FillBytesPayloadOverrides { length: Some(48) });

    // Act
    let adapter: Box<dyn IRandomSourceAdapter> = create_random_source(
        &build_create_random_source_deps(Default::default()),
        params,
        build_create_random_source_payload(Default::default()),
    )
    .expect("the mock selection constructs its source")
    .adapter;
    let result = adapter.fill_bytes(FillBytesParams, payload);

    // Assert
    let success = result.expect("the succeeds-mode source fills the buffer");
    assert_eq!(success.bytes.expose().len(), 48);
}
// create_random_source_returns_the_mock_source_drawing_the_bytes_requested_in_the_succeeds_mode
