#![allow(clippy::expect_used)]

use super::provides::{
    CreateRandomSourceParamsOverrides, FillBytesErrorReturn, FillBytesParams,
    FillBytesPayloadOverrides, IRandomSourceAdapter, MockIRandomSourceAdapterFailureMode,
    RANDOM_SOURCE_KINDS, RandomSourceKind, build_create_random_source_deps,
    build_create_random_source_params, build_create_random_source_payload,
    build_fill_bytes_payload, build_random_source_kind, create_random_source,
};

/// Contract: own entry
///   `every_source_the_factory_constructs_draws_exactly_the_bytes_requested` —
///   `fill_bytes` is called on the source `create_random_source` returns for
///   each selection `build_random_source_kind` returns for a member of
///   `RANDOM_SOURCE_KINDS` and for `RandomSourceKind::Mock(
///   MockIRandomSourceAdapterFailureMode::Succeeds)`, for a length of 48 →
///   `Ok` whose `bytes` hold exactly 48 bytes.
/// Boundary: `create_random_source` → the selection's `try_new` →
///   `IRandomSourceAdapter::fill_bytes`.
/// Mocked:  nothing; the operating system's generator is the outer edge and
///   runs real.
/// Arrange: `build_create_random_source_params` with the kind override set to
///   each selection the block runs over; `build_fill_bytes_payload` with the
///   length override set to 48, which differs from the builder's default of
///   32.
/// Act:     `create_random_source` over
///   `build_create_random_source_deps(Default::default())`, the params, and
///   `build_create_random_source_payload(Default::default())`, then
///   `fill_bytes(FillBytesParams, payload)` on the success arm's adapter.
/// Assert:  the adapter and the `fill_bytes` success arm are each extracted
///   with `expect`; the length of `bytes.expose()` equals the literal 48
///   written in the assertion.
#[test]
fn every_source_the_factory_constructs_draws_exactly_the_bytes_requested() {
    for kind in RANDOM_SOURCE_KINDS
        .iter()
        .map(|member| build_random_source_kind(*member, Default::default()))
        .chain([RandomSourceKind::Mock(
            MockIRandomSourceAdapterFailureMode::Succeeds,
        )])
    {
        // Arrange
        let params = build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(kind),
        });
        let payload = build_fill_bytes_payload(FillBytesPayloadOverrides { length: Some(48) });

        // Act
        let adapter: Box<dyn IRandomSourceAdapter> = create_random_source(
            &build_create_random_source_deps(Default::default()),
            params,
            build_create_random_source_payload(Default::default()),
        )
        .expect("every selection the family admits constructs its source")
        .adapter;
        let result = adapter.fill_bytes(FillBytesParams, payload);

        // Assert
        let success = result.expect("every source the factory constructs fills the buffer");
        assert_eq!(success.bytes.expose().len(), 48);
    }
}
// every_source_the_factory_constructs_draws_exactly_the_bytes_requested

/// Contract: own entry
///   `every_source_the_factory_constructs_draws_an_empty_buffer_for_a_zero_length` —
///   `fill_bytes` is called on the source `create_random_source` returns for
///   each selection `build_random_source_kind` returns for a member of
///   `RANDOM_SOURCE_KINDS` and for `RandomSourceKind::Mock(
///   MockIRandomSourceAdapterFailureMode::Succeeds)`, for a length of zero →
///   `Ok` whose `bytes` are empty.
/// Boundary: `create_random_source` → the selection's `try_new` →
///   `IRandomSourceAdapter::fill_bytes`.
/// Mocked:  nothing; the operating system's generator is the outer edge and
///   runs real.
/// Arrange: `build_create_random_source_params` with the kind override set to
///   each selection the block runs over; `build_fill_bytes_payload` with the
///   length override set to 0, which differs from the builder's default of
///   32.
/// Act:     `create_random_source` over the deps, the params, and the
///   payload, then `fill_bytes(FillBytesParams, payload)` on the success
///   arm's adapter.
/// Assert:  the adapter and the `fill_bytes` success arm are each extracted
///   with `expect`; `bytes.expose()` is empty.
#[test]
fn every_source_the_factory_constructs_draws_an_empty_buffer_for_a_zero_length() {
    for kind in RANDOM_SOURCE_KINDS
        .iter()
        .map(|member| build_random_source_kind(*member, Default::default()))
        .chain([RandomSourceKind::Mock(
            MockIRandomSourceAdapterFailureMode::Succeeds,
        )])
    {
        // Arrange
        let params = build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(kind),
        });
        let payload = build_fill_bytes_payload(FillBytesPayloadOverrides { length: Some(0) });

        // Act
        let adapter: Box<dyn IRandomSourceAdapter> = create_random_source(
            &build_create_random_source_deps(Default::default()),
            params,
            build_create_random_source_payload(Default::default()),
        )
        .expect("every selection the family admits constructs its source")
        .adapter;
        let result = adapter.fill_bytes(FillBytesParams, payload);

        // Assert
        let success = result.expect("every source the factory constructs fills the buffer");
        assert!(success.bytes.expose().is_empty());
    }
}
// every_source_the_factory_constructs_draws_an_empty_buffer_for_a_zero_length

/// Contract: own entry
///   `the_mock_source_returns_the_family_mock_error_when_its_failure_mode_refuses` —
///   `fill_bytes` is called on the source `create_random_source` returns for
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)` →
///   `Err(FillBytesErrorReturn::MockIRandomSourceAdapter)`.
/// Boundary: `create_random_source` → the selection's `try_new` →
///   `IRandomSourceAdapter::fill_bytes`.
/// Mocked:  nothing; the mock concrete is the family's own, and the declared
///   set cannot reach this arm, so the block runs over the mock selection
///   alone.
/// Arrange: `build_create_random_source_params` with the kind override set to
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)`, which differs from the builder's default `Succeeds`;
///   `build_fill_bytes_payload` at its defaults.
/// Act:     `create_random_source` over the deps, the params, and the
///   payload, then `fill_bytes(FillBytesParams, payload)` on the success
///   arm's adapter.
/// Assert:  the adapter is extracted from the success arm with `expect`; the
///   `fill_bytes` result's `err()` equals
///   `Some(FillBytesErrorReturn::MockIRandomSourceAdapter)` under
///   `assert_eq!`.
#[test]
fn the_mock_source_returns_the_family_mock_error_when_its_failure_mode_refuses() {
    // Arrange
    let params = build_create_random_source_params(CreateRandomSourceParamsOverrides {
        kind: Some(RandomSourceKind::Mock(
            MockIRandomSourceAdapterFailureMode::FillBytesRefused,
        )),
    });
    let payload = build_fill_bytes_payload(Default::default());

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
    assert_eq!(
        result.err(),
        Some(FillBytesErrorReturn::MockIRandomSourceAdapter)
    );
}
// the_mock_source_returns_the_family_mock_error_when_its_failure_mode_refuses
