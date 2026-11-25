#![allow(clippy::expect_used)]

use super::provides::{
    ConsumeRandomSourceParams, ConsumeRandomSourcePayload, CreateRandomSourceParamsOverrides,
    FillBytesErrorReturn, FillBytesParams, FillBytesPayload, FillBytesPayloadOverrides,
    FillBytesReturn, IRandomSourceAdapter, IRandomSourceConsumer,
    MockIRandomSourceAdapterFailureMode, RANDOM_SOURCE_KINDS, RandomSourceKind,
    build_create_random_source_deps, build_create_random_source_params,
    build_create_random_source_payload, build_fill_bytes_payload, build_random_source_kind,
    create_random_source,
};

/// Contract: own entry
///   `every_source_the_factory_constructs_draws_exactly_the_bytes_requested`.
/// Boundary: `create_random_source` → the selection's `try_new` →
///   `IRandomSourceConsumer::consume_random_source` →
///   `IRandomSourceAdapter::fill_bytes`.
/// Mocked:  nothing; the operating system's generator is the outer edge and
///   runs real.
/// Arrange: `build_create_random_source_params` with the kind override set
///   to each selection the block runs over; `build_fill_bytes_payload` with
///   the length override set to 48, which differs from the builder's default
///   of 32; the block's consumer, whose `consume_random_source` calls
///   `fill_bytes(FillBytesParams, payload)` on the source it receives and
///   returns the `FillBytesReturn` it observes.
/// Act:     `create_random_source` over
///   `build_create_random_source_deps(consumer, Default::default())`, the
///   params, and `build_create_random_source_payload(Default::default())`.
/// Assert:  the factory's success arm is extracted with `expect`; the
///   `fill_bytes` success the consumer's output holds is extracted with
///   `expect`, and the length of `bytes.expose()` equals the literal 48
///   written in the assertion.
#[test]
fn every_source_the_factory_constructs_draws_exactly_the_bytes_requested() {
    struct DrawingConsumer {
        payload: FillBytesPayload,
    }

    impl IRandomSourceConsumer for DrawingConsumer {
        type Output = FillBytesReturn;

        fn consume_random_source<K: IRandomSourceAdapter>(
            &self,
            _params: ConsumeRandomSourceParams,
            payload: ConsumeRandomSourcePayload<K>,
        ) -> Self::Output {
            payload.source.fill_bytes(
                FillBytesParams,
                FillBytesPayload {
                    length: self.payload.length,
                },
            )
        }
    }

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
        let deps = build_create_random_source_deps(
            DrawingConsumer {
                payload: build_fill_bytes_payload(FillBytesPayloadOverrides { length: Some(48) }),
            },
            Default::default(),
        );

        // Act
        let result = create_random_source(
            &deps,
            params,
            build_create_random_source_payload(Default::default()),
        );

        // Assert
        let success = result.expect("every selection the family admits constructs its source");
        let fill_success = success
            .output
            .expect("every source the factory constructs fills the buffer");
        assert_eq!(fill_success.bytes.expose().len(), 48);
    }
}
// every_source_the_factory_constructs_draws_exactly_the_bytes_requested

/// Contract: own entry
///   `every_source_the_factory_constructs_draws_an_empty_buffer_for_a_zero_length`.
/// Boundary: `create_random_source` → the selection's `try_new` →
///   `IRandomSourceConsumer::consume_random_source` →
///   `IRandomSourceAdapter::fill_bytes`.
/// Mocked:  nothing; the operating system's generator is the outer edge and
///   runs real.
/// Arrange: `build_create_random_source_params` with the kind override set
///   to each selection the block runs over; `build_fill_bytes_payload` with
///   the length override set to 0, which differs from the builder's default
///   of 32; the block's consumer, whose `consume_random_source` calls
///   `fill_bytes(FillBytesParams, payload)` on the source it receives and
///   returns the `FillBytesReturn` it observes.
/// Act:     `create_random_source` over the deps, the params, and the
///   payload.
/// Assert:  the factory's success arm is extracted with `expect`; the
///   `fill_bytes` success the consumer's output holds is extracted with
///   `expect`, and `bytes.expose()` is empty.
#[test]
fn every_source_the_factory_constructs_draws_an_empty_buffer_for_a_zero_length() {
    struct DrawingConsumer {
        payload: FillBytesPayload,
    }

    impl IRandomSourceConsumer for DrawingConsumer {
        type Output = FillBytesReturn;

        fn consume_random_source<K: IRandomSourceAdapter>(
            &self,
            _params: ConsumeRandomSourceParams,
            payload: ConsumeRandomSourcePayload<K>,
        ) -> Self::Output {
            payload.source.fill_bytes(
                FillBytesParams,
                FillBytesPayload {
                    length: self.payload.length,
                },
            )
        }
    }

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
        let deps = build_create_random_source_deps(
            DrawingConsumer {
                payload: build_fill_bytes_payload(FillBytesPayloadOverrides { length: Some(0) }),
            },
            Default::default(),
        );

        // Act
        let result = create_random_source(
            &deps,
            params,
            build_create_random_source_payload(Default::default()),
        );

        // Assert
        let success = result.expect("every selection the family admits constructs its source");
        let fill_success = success
            .output
            .expect("every source the factory constructs fills the buffer");
        assert!(fill_success.bytes.expose().is_empty());
    }
}
// every_source_the_factory_constructs_draws_an_empty_buffer_for_a_zero_length

/// Contract: own entry
///   `the_mock_source_returns_the_family_mock_error_when_its_failure_mode_refuses`.
/// Boundary: `create_random_source` → the selection's `try_new` →
///   `IRandomSourceConsumer::consume_random_source` →
///   `IRandomSourceAdapter::fill_bytes`.
/// Mocked:  nothing; the mock concrete is the family's own, and the declared
///   set cannot reach this arm, so the block runs over the mock selection
///   alone.
/// Arrange: `build_create_random_source_params` with the kind override set
///   to `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)`, which differs from the builder's default
///   `Succeeds`; `build_fill_bytes_payload` at its defaults; the block's
///   consumer, whose `consume_random_source` calls
///   `fill_bytes(FillBytesParams, payload)` on the source it receives and
///   returns the `FillBytesReturn` it observes.
/// Act:     `create_random_source` over the deps, the params, and the
///   payload.
/// Assert:  the factory's success arm is extracted with `expect`; the
///   `err()` of the `FillBytesReturn` the consumer's output holds equals
///   `Some(FillBytesErrorReturn::MockIRandomSourceAdapter)` under
///   `assert_eq!`.
#[test]
fn the_mock_source_returns_the_family_mock_error_when_its_failure_mode_refuses() {
    struct DrawingConsumer {
        payload: FillBytesPayload,
    }

    impl IRandomSourceConsumer for DrawingConsumer {
        type Output = FillBytesReturn;

        fn consume_random_source<K: IRandomSourceAdapter>(
            &self,
            _params: ConsumeRandomSourceParams,
            payload: ConsumeRandomSourcePayload<K>,
        ) -> Self::Output {
            payload.source.fill_bytes(
                FillBytesParams,
                FillBytesPayload {
                    length: self.payload.length,
                },
            )
        }
    }

    // Arrange
    let params = build_create_random_source_params(CreateRandomSourceParamsOverrides {
        kind: Some(RandomSourceKind::Mock(
            MockIRandomSourceAdapterFailureMode::FillBytesRefused,
        )),
    });
    let deps = build_create_random_source_deps(
        DrawingConsumer {
            payload: build_fill_bytes_payload(Default::default()),
        },
        Default::default(),
    );

    // Act
    let result = create_random_source(
        &deps,
        params,
        build_create_random_source_payload(Default::default()),
    );

    // Assert
    let success = result.expect("the mock selection constructs its source");
    assert_eq!(
        success.output.err(),
        Some(FillBytesErrorReturn::MockIRandomSourceAdapter)
    );
}
// the_mock_source_returns_the_family_mock_error_when_its_failure_mode_refuses
