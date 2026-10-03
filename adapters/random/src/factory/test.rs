#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::create_random_source;
use super::interface::{
    CreateRandomSourceDeps, CreateRandomSourcePayload, RANDOM_SOURCE_INTERFACE_VERSION,
    RandomSourceKind,
};
use super::mock::{CreateRandomSourceParamsOverrides, build_create_random_source_params};

/// Contract: given params.kind is RandomSourceKind::OperatingSystem, the
///   factory calls OsRandomSource::try_new once and returns Ok holding the
///   concrete boxed as dyn IRandomSourceAdapter, which reports its own
///   declaration through the trait.
/// Arrange: params built with `kind: Some(RandomSourceKind::OperatingSystem)`.
/// Act:     `create_random_source(&CreateRandomSourceDeps, params, CreateRandomSourcePayload)`.
/// Assert:  `success.adapter.declaration().source` matches
///   `RandomSourceKind::OperatingSystem`, `adapter_version` equals 1, and
///   `interface_version` equals `RANDOM_SOURCE_INTERFACE_VERSION`.
#[test]
fn create_random_source_returns_the_operating_system_source_for_its_kind() {
    // Arrange
    let params = build_create_random_source_params(CreateRandomSourceParamsOverrides {
        kind: Some(RandomSourceKind::OperatingSystem),
    });

    // Act
    let Ok(success) =
        create_random_source(&CreateRandomSourceDeps, params, CreateRandomSourcePayload);

    // Assert
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
