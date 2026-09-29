#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::create_random_source;
use super::interface::{
    CreateRandomSourceDeps, CreateRandomSourceParams, RANDOM_SOURCE_INTERFACE_VERSION,
    RandomSourceKind,
};
use super::mock::{CreateRandomSourcePayloadOverrides, build_create_random_source_payload};

/// Contract: given payload.kind is RandomSourceKind::OperatingSystem, the
///   factory calls OsRandomSource::try_new once and returns Ok holding the
///   concrete boxed as dyn IRandomSourceAdapter beside OsRandomSource::DECLARATION.
/// Arrange: a payload built with `kind: Some(RandomSourceKind::OperatingSystem)`.
/// Act:     `create_random_source(&CreateRandomSourceDeps, CreateRandomSourceParams, payload)`.
/// Assert:  `success.declaration.adapter_version` equals 1 and
///   `success.declaration.interface_version` equals
///   `RANDOM_SOURCE_INTERFACE_VERSION`.
#[test]
fn create_random_source_returns_the_operating_system_source_for_its_kind() {
    // Arrange
    let payload = build_create_random_source_payload(CreateRandomSourcePayloadOverrides {
        kind: Some(RandomSourceKind::OperatingSystem),
    });

    // Act
    let Ok(success) =
        create_random_source(&CreateRandomSourceDeps, CreateRandomSourceParams, payload);

    // Assert
    assert_eq!(success.declaration.adapter_version, 1);
    assert_eq!(
        success.declaration.interface_version,
        RANDOM_SOURCE_INTERFACE_VERSION
    );
}
