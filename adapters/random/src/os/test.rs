#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{OsRandomSource, OsRandomSourceConstructorParams};
use crate::factory::provides::{
    FillBytesParams, FillBytesPayloadOverrides, IRandomSourceAdapter, MockIRandomSourceAdapter,
    RANDOM_SOURCE_INTERFACE_VERSION, build_fill_bytes_payload,
};
use std::collections::HashSet;

/// Contract: given a fill_bytes call whose generator fill returns Ok over a
///   buffer of payload.length bytes, the success return holds the filled
///   buffer — a payload length selects the draw's length.
/// Arrange: the operating-system source and a payload of length 48, differing
///   from the builder's default of 32.
/// Act:     `source.fill_bytes(FillBytesParams, payload)`.
/// Assert:  the exposed draw's length equals 48.
#[test]
fn fill_bytes_returns_the_number_of_bytes_requested() {
    // Arrange
    let Ok(source) = OsRandomSource::try_new(OsRandomSourceConstructorParams);
    let payload = build_fill_bytes_payload(FillBytesPayloadOverrides { length: Some(48) });

    // Act
    let Ok(success) = source.fill_bytes(FillBytesParams, payload) else {
        panic!("the operating-system source draws successfully")
    };

    // Assert
    assert_eq!(success.bytes.expose().len(), 48);
}

/// Contract: a payload.length of zero takes the drawn branch with an empty
///   buffer.
/// Arrange: the operating-system source and a payload of length 0.
/// Act:     `source.fill_bytes(FillBytesParams, payload)`.
/// Assert:  the call returns Ok and the exposed draw is empty.
#[test]
fn fill_bytes_returns_an_empty_draw_for_a_zero_length() {
    // Arrange
    let Ok(source) = OsRandomSource::try_new(OsRandomSourceConstructorParams);
    let payload = build_fill_bytes_payload(FillBytesPayloadOverrides { length: Some(0) });

    // Act
    let result = source.fill_bytes(FillBytesParams, payload);

    // Assert
    let Ok(success) = result else {
        panic!("a zero-length draw takes the drawn branch")
    };
    assert!(success.bytes.expose().is_empty());
}

/// Contract: draws from the generator do not repeat — repeated draws of a
///   fixed width are pairwise distinct across a fixed count.
/// Arrange: the operating-system source and an empty set of draws.
/// Act:     `source.fill_bytes(FillBytesParams, build_fill_bytes_payload(Default::default()))`
///   sixteen times, inserting a copy of each exposed draw into the set.
/// Assert:  the set holds sixteen entries.
#[test]
fn fill_bytes_draws_pairwise_distinct_values_across_repeated_draws() {
    // Arrange
    let Ok(source) = OsRandomSource::try_new(OsRandomSourceConstructorParams);
    let mut draws: HashSet<Vec<u8>> = HashSet::new();

    // Act
    for _ in 0..16 {
        let Ok(success) = source.fill_bytes(
            FillBytesParams,
            build_fill_bytes_payload(Default::default()),
        ) else {
            panic!("the operating-system source draws successfully")
        };
        draws.insert(success.bytes.expose().clone());
    }

    // Assert
    assert_eq!(draws.len(), 16);
}

/// Contract: a draw is filled by the generator rather than left at its zero
///   initialization — a draw of a fixed length holds more than one distinct
///   byte value.
/// Arrange: the operating-system source and a payload of length 1024.
/// Act:     `source.fill_bytes(FillBytesParams, payload)`.
/// Assert:  the set of distinct byte values in the exposed draw holds more
///   than one value.
#[test]
fn fill_bytes_fills_a_draw_with_more_than_one_distinct_byte_value() {
    // Arrange
    let Ok(source) = OsRandomSource::try_new(OsRandomSourceConstructorParams);
    let payload = build_fill_bytes_payload(FillBytesPayloadOverrides { length: Some(1024) });

    // Act
    let Ok(success) = source.fill_bytes(FillBytesParams, payload) else {
        panic!("the operating-system source draws successfully")
    };

    // Assert
    let distinct: HashSet<u8> = success.bytes.expose().iter().copied().collect();
    assert!(distinct.len() > 1);
}

/// Contract: the concrete's declaration names its adapter version and the
///   interface version it implements, readable from the type before any
///   instance exists.
/// Arrange: nothing.
/// Act:     read `OsRandomSource::DECLARATION`.
/// Assert:  `adapter_version` equals 1 and `interface_version` equals
///   `RANDOM_SOURCE_INTERFACE_VERSION`.
#[test]
fn os_random_source_declares_its_adapter_and_interface_versions() {
    // Arrange

    // Act
    let declaration = OsRandomSource::DECLARATION;

    // Assert
    assert_eq!(declaration.adapter_version, 1);
    assert_eq!(
        declaration.interface_version,
        RANDOM_SOURCE_INTERFACE_VERSION
    );
}

/// Contract: the default family mock obeys the same length contract as the
///   operating-system source — a payload length selects the draw's length,
///   including zero.
/// Arrange: `MockIRandomSourceAdapter` and payloads of zero and 48 bytes.
/// Act:     `fill_bytes` for each payload.
/// Assert:  the exposed draws have lengths zero and 48 respectively.
#[test]
fn mock_fill_bytes_honors_the_requested_length() {
    // Arrange
    let source = MockIRandomSourceAdapter;
    let empty = build_fill_bytes_payload(FillBytesPayloadOverrides { length: Some(0) });
    let forty_eight = build_fill_bytes_payload(FillBytesPayloadOverrides { length: Some(48) });

    // Act
    let Ok(empty_success) = source.fill_bytes(FillBytesParams, empty) else {
        panic!("the default mock draws successfully")
    };
    let Ok(success) = source.fill_bytes(FillBytesParams, forty_eight) else {
        panic!("the default mock draws successfully")
    };

    // Assert
    assert!(empty_success.bytes.expose().is_empty());
    assert_eq!(success.bytes.expose().len(), 48);
}
