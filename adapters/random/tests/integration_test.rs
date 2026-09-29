#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use random::{
    CreateRandomSourceDeps, CreateRandomSourceParams, CreateRandomSourcePayloadOverrides,
    FillBytesParams, FillBytesPayloadOverrides, RandomSourceKind,
    build_create_random_source_payload, build_fill_bytes_payload, create_random_source,
};
use std::collections::HashSet;

/// Contract: the factory's operating-system source, used only through
///   Box<dyn IRandomSourceAdapter>, returns a draw of the requested length
///   filled by the generator — a production draw passes through the family's
///   surface (CR-05).
/// Arrange: a payload built with `kind: Some(RandomSourceKind::OperatingSystem)`
///   and a fill payload of `length: Some(64)`.
/// Act:     `create_random_source` and then `fill_bytes` on the returned
///   adapter.
/// Assert:  the exposed draw's length equals 64 and the `HashSet<u8>` of its
///   bytes holds more than one value.
#[test]
fn a_source_from_the_factory_draws_random_bytes_through_the_family_trait() {
    // Arrange
    let payload = build_create_random_source_payload(CreateRandomSourcePayloadOverrides {
        kind: Some(RandomSourceKind::OperatingSystem),
    });
    let fill_payload = build_fill_bytes_payload(FillBytesPayloadOverrides { length: Some(64) });

    // Act
    let Ok(success) =
        create_random_source(&CreateRandomSourceDeps, CreateRandomSourceParams, payload);
    let adapter = success.adapter;
    let Ok(draw) = adapter.fill_bytes(FillBytesParams, fill_payload) else {
        panic!("the operating-system source draws through the family trait")
    };

    // Assert
    assert_eq!(draw.bytes.expose().len(), 64);
    let distinct: HashSet<u8> = draw.bytes.expose().iter().copied().collect();
    assert!(distinct.len() > 1);
}
