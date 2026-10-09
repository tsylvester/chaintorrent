#![allow(clippy::expect_used)]

use super::provides::{
    CreateRandomSourceDeps, CreateRandomSourceParamsOverrides, CreateRandomSourcePayload,
    MockIRandomSourceAdapterFailureMode, RANDOM_SOURCE_INTERFACE_VERSION, RandomSourceKind,
    build_create_random_source_params, create_random_source,
};

/// Contract: `params.kind` is `RandomSourceKind::OperatingSystem` →
///   `Ok(CreateRandomSourceSuccessReturn { adapter })` holding the
///   operating-system concrete, which reports `OsRandomSource::DECLARATION`
///   through `declaration()`.
/// Arrange: `build_create_random_source_params` with the kind override set to
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `create_random_source(&CreateRandomSourceDeps, params,
///   CreateRandomSourcePayload)`.
/// Assert:  the success arm is extracted with `expect`; `adapter.declaration()`
///   has `source` matching `RandomSourceKind::OperatingSystem` under `matches!`,
///   `adapter_version` equal to the literal 1 written in the assertion, and
///   `interface_version` equal to `RANDOM_SOURCE_INTERFACE_VERSION`.
#[test]
fn create_random_source_returns_the_operating_system_source_for_its_kind() {
    // Arrange
    let params = build_create_random_source_params(CreateRandomSourceParamsOverrides {
        kind: Some(RandomSourceKind::OperatingSystem),
    });

    // Act
    let result = create_random_source(&CreateRandomSourceDeps, params, CreateRandomSourcePayload);

    // Assert
    let success = result.expect("the operating-system kind constructs its source");
    let declaration = success.adapter.declaration();
    assert!(matches!(
        declaration.source,
        RandomSourceKind::OperatingSystem
    ));
    assert_eq!(declaration.adapter_version, 1);
    assert_eq!(
        declaration.interface_version,
        RANDOM_SOURCE_INTERFACE_VERSION
    );
}

/// Contract: `params.kind` is `RandomSourceKind::Mock(mode)` →
///   `Ok(CreateRandomSourceSuccessReturn { adapter })` holding the mock
///   concrete, whose declaration's `source` is `RandomSourceKind::Mock(mode)`.
/// Arrange: `build_create_random_source_params` with the kind override set to
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)`,
///   a mode that differs from the builder's default `Succeeds`, so an arm that
///   drops the mode fails.
/// Act:     `create_random_source(&CreateRandomSourceDeps, params,
///   CreateRandomSourcePayload)`.
/// Assert:  the success arm is extracted with `expect`;
///   `adapter.declaration().source` matches
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)`
///   under `matches!`.
#[test]
fn create_random_source_returns_the_mock_source_carrying_the_failure_mode_its_kind_names() {
    // Arrange
    let params = build_create_random_source_params(CreateRandomSourceParamsOverrides {
        kind: Some(RandomSourceKind::Mock(
            MockIRandomSourceAdapterFailureMode::FillBytesRefused,
        )),
    });

    // Act
    let result = create_random_source(&CreateRandomSourceDeps, params, CreateRandomSourcePayload);

    // Assert
    let success = result.expect("the mock kind constructs its source");
    assert!(matches!(
        success.adapter.declaration().source,
        RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)
    ));
}
