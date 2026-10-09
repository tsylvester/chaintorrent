#![allow(clippy::expect_used)]

use super::provides::{OsRandomSource, OsRandomSourceConstructorParams, build_os_random_source};
use crate::factory::provides::{
    FillBytesParams, FillBytesPayloadOverrides, IRandomSourceAdapter,
    RANDOM_SOURCE_INTERFACE_VERSION, RandomSourceKind, build_fill_bytes_payload,
};
use std::collections::HashSet;

/// Contract: `getrandom::fill` returns `Ok(())` over a buffer of `payload.length`
///   bytes → `Ok(FillBytesSuccessReturn)` whose `Secret` holds exactly
///   `payload.length` bytes.
/// Arrange: `build_fill_bytes_payload` with the length override set to 48, a
///   nonzero length that differs from the builder's default of 32, so a source
///   returning the default width fails.
/// Act:     `fill_bytes(FillBytesParams, payload)` on the built source.
/// Assert:  the success arm is extracted with `expect`; the length of
///   `bytes.expose()` equals the literal 48 written in the assertion.
#[test]
fn fill_bytes_returns_the_number_of_bytes_requested() {
    // Arrange
    let source = build_os_random_source();
    let payload = build_fill_bytes_payload(FillBytesPayloadOverrides { length: Some(48) });

    // Act
    let result = source.fill_bytes(FillBytesParams, payload);

    // Assert
    let success = result.expect("the operating-system source fills the buffer");
    assert_eq!(success.bytes.expose().len(), 48);
}

/// Contract: `payload.length` of zero takes the drawn branch with an empty
///   buffer → `Ok(FillBytesSuccessReturn)` whose `Secret` holds no bytes.
/// Arrange: `build_fill_bytes_payload` with the length override set to 0, so a
///   source that ignores the length and returns the builder's default width
///   fails.
/// Act:     `fill_bytes(FillBytesParams, payload)` on the built source.
/// Assert:  the success arm is extracted with `expect`; `bytes.expose()` is
///   empty.
#[test]
fn fill_bytes_returns_an_empty_draw_for_a_zero_length() {
    // Arrange
    let source = build_os_random_source();
    let payload = build_fill_bytes_payload(FillBytesPayloadOverrides { length: Some(0) });

    // Act
    let result = source.fill_bytes(FillBytesParams, payload);

    // Assert
    let success = result.expect("the operating-system source fills the buffer");
    assert!(success.bytes.expose().is_empty());
}

/// Contract: a drawn buffer is filled by the generator → segments of a fixed
///   width within one draw are pairwise distinct.
/// Arrange: `build_fill_bytes_payload` with the length override set to 1024,
///   which splits into segments of 32 bytes, so a zero-filled or repeating
///   buffer yields fewer distinct segments than segments.
/// Act:     `fill_bytes(FillBytesParams, payload)` on the built source.
/// Assert:  the success arm is extracted with `expect`; the 32-byte segments of
///   `bytes.expose()` collected into a `HashSet` number the literal 32 written
///   in the assertion.
#[test]
fn fill_bytes_draws_pairwise_distinct_segments_within_a_draw() {
    // Arrange
    let source = build_os_random_source();
    let payload = build_fill_bytes_payload(FillBytesPayloadOverrides { length: Some(1024) });

    // Act
    let result = source.fill_bytes(FillBytesParams, payload);

    // Assert
    let success = result.expect("the operating-system source fills the buffer");
    let segments: HashSet<&[u8]> = success.bytes.expose().chunks(32).collect();
    assert_eq!(segments.len(), 32);
}

/// Contract: a drawn buffer is filled by the generator → a draw of a fixed
///   length holds more than one distinct byte value.
/// Arrange: `build_fill_bytes_payload` with the length override set to 256, so
///   a zero-filled buffer holds one distinct byte value.
/// Act:     `fill_bytes(FillBytesParams, payload)` on the built source.
/// Assert:  the success arm is extracted with `expect`; the bytes of
///   `bytes.expose()` collected into a `HashSet` number more than the literal 1
///   written in the assertion.
#[test]
fn fill_bytes_fills_a_draw_with_more_than_one_distinct_byte_value() {
    // Arrange
    let source = build_os_random_source();
    let payload = build_fill_bytes_payload(FillBytesPayloadOverrides { length: Some(256) });

    // Act
    let result = source.fill_bytes(FillBytesParams, payload);

    // Assert
    let success = result.expect("the operating-system source fills the buffer");
    let distinct: HashSet<&u8> = success.bytes.expose().iter().collect();
    assert!(distinct.len() > 1);
}

/// Contract: `OsRandomSource::DECLARATION` is `RandomSourceDeclaration {
///   source: RandomSourceKind::OperatingSystem, adapter_version: 1,
///   interface_version: RANDOM_SOURCE_INTERFACE_VERSION }`, readable from the
///   type before any instance exists.
/// Arrange: none; the type alone.
/// Act:     reading `OsRandomSource::DECLARATION`.
/// Assert:  `source` matches `RandomSourceKind::OperatingSystem` under
///   `matches!`; `adapter_version` equals the literal 1 written in the
///   assertion; `interface_version` equals `RANDOM_SOURCE_INTERFACE_VERSION`.
#[test]
fn os_random_source_declares_its_adapter_and_interface_versions() {
    // Arrange

    // Act
    let declaration = OsRandomSource::DECLARATION;

    // Assert
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

/// Contract: `declaration()` on a living source → `Self::DECLARATION`.
/// Arrange: `build_os_random_source()`.
/// Act:     `declaration()` on the built source through `IRandomSourceAdapter`.
/// Assert:  `source` matches `RandomSourceKind::OperatingSystem` under
///   `matches!`; `adapter_version` equals the literal 1 written in the
///   assertion; `interface_version` equals `RANDOM_SOURCE_INTERFACE_VERSION`.
#[test]
fn declaration_returns_the_associated_declaration() {
    // Arrange
    let source = build_os_random_source();

    // Act
    let declaration = source.declaration();

    // Assert
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

/// Contract: any params → `Ok(OsRandomSource)`.
/// Arrange: `OsRandomSourceConstructorParams` by its production value.
/// Act:     `OsRandomSource::try_new(OsRandomSourceConstructorParams)`.
/// Assert:  `result.is_ok()` is true.
#[test]
fn try_new_returns_the_operating_system_source() {
    // Arrange

    // Act
    let result = OsRandomSource::try_new(OsRandomSourceConstructorParams);

    // Assert
    assert!(result.is_ok());
}
