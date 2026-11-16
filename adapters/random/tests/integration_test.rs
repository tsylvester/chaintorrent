#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use random::{
    CreateRandomSourceParamsOverrides, FillBytesParams, FillBytesPayloadOverrides,
    RANDOM_SOURCE_INTERFACE_VERSION, RANDOM_SOURCE_KINDS, RandomSourceKind,
    build_create_random_source_deps, build_create_random_source_params,
    build_create_random_source_payload, build_fill_bytes_payload, create_random_source,
};
use std::collections::HashSet;

/// Contract: a source obtained from `create_random_source` for each member of
///   `RANDOM_SOURCE_KINDS` draws the requested number of bytes through
///   `fill_bytes`, and the draw is filled by that member's generator.
/// Boundary: the crate's public surface; the real chain `create_random_source`
///   → the member's `try_new` → `IRandomSourceAdapter::fill_bytes` → the
///   member's generator.
/// Mocked:   nothing; each member's generator is its outer edge and runs real,
///   so this test proves the generator's output reaches the caller and does not
///   prove the generator's quality.
/// Arrange:  the adapter returned by `create_random_source` for each member of
///   `RANDOM_SOURCE_KINDS` in turn, `build_create_random_source_params`
///   carrying the kind override set to the member, and
///   `build_create_random_source_deps(Default::default())` and
///   `build_create_random_source_payload(Default::default())` as its deps and
///   payload; `build_fill_bytes_payload` with the length override set to 1024,
///   which splits into segments of 32 bytes and differs from the builder's
///   default of 32.
/// Act:      `fill_bytes(FillBytesParams, payload)` on the returned adapter.
/// Assert:   the success arm is extracted with `expect`; the length of
///   `bytes.expose()` equals the literal 1024 written in the assertion; the
///   32-byte segments of `bytes.expose()` collected into a `HashSet` number the
///   literal 32 written in the assertion.
#[test]
fn a_source_from_the_factory_draws_random_bytes_through_the_family_trait() {
    for kind in RANDOM_SOURCE_KINDS {
        // Arrange
        let kind: RandomSourceKind = *kind;
        let params = build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(kind),
        });
        let adapter = create_random_source(
            &build_create_random_source_deps(Default::default()),
            params,
            build_create_random_source_payload(Default::default()),
        )
        .expect("every member of the declared set constructs its source")
        .adapter;
        let payload = build_fill_bytes_payload(FillBytesPayloadOverrides { length: Some(1024) });

        // Act
        let result = adapter.fill_bytes(FillBytesParams, payload);

        // Assert
        let success = result.expect("the member's source fills the buffer");
        assert_eq!(success.bytes.expose().len(), 1024);
        let segments: HashSet<&[u8]> = success.bytes.expose().chunks(32).collect();
        assert_eq!(segments.len(), 32);
    }
}

/// Contract: a source obtained from `create_random_source` for each member of
///   `RANDOM_SOURCE_KINDS` draws zero bytes for a `payload.length` of zero
///   through `fill_bytes`.
/// Boundary: the crate's public surface; the real chain `create_random_source`
///   → the member's `try_new` → `IRandomSourceAdapter::fill_bytes` → the
///   member's generator.
/// Mocked:   nothing; each member's generator is its outer edge and runs real.
/// Arrange:  the adapter returned by `create_random_source` for each member of
///   `RANDOM_SOURCE_KINDS` in turn, `build_create_random_source_params`
///   carrying the kind override set to the member, and
///   `build_create_random_source_deps(Default::default())` and
///   `build_create_random_source_payload(Default::default())` as its deps and
///   payload; `build_fill_bytes_payload` with the length override set to 0,
///   which differs from the builder's default of 32.
/// Act:      `fill_bytes(FillBytesParams, payload)` on the returned adapter.
/// Assert:   the success arm is extracted with `expect`; `bytes.expose()` is
///   empty.
#[test]
fn a_source_from_the_factory_draws_an_empty_buffer_for_a_zero_length() {
    for kind in RANDOM_SOURCE_KINDS {
        // Arrange
        let kind: RandomSourceKind = *kind;
        let params = build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(kind),
        });
        let adapter = create_random_source(
            &build_create_random_source_deps(Default::default()),
            params,
            build_create_random_source_payload(Default::default()),
        )
        .expect("every member of the declared set constructs its source")
        .adapter;
        let payload = build_fill_bytes_payload(FillBytesPayloadOverrides { length: Some(0) });

        // Act
        let result = adapter.fill_bytes(FillBytesParams, payload);

        // Assert
        let success = result.expect("the member's source fills the buffer");
        assert!(success.bytes.expose().is_empty());
    }
}

/// Contract: a source obtained from `create_random_source` for each member of
///   `RANDOM_SOURCE_KINDS` reports, through `declaration()`, the declaration of
///   the concrete its `kind` names.
/// Boundary: the crate's public surface; the real chain `create_random_source`
///   → the member's `try_new` → `IRandomSourceAdapter::declaration`.
/// Mocked:   nothing; the chain has no outer edge beyond the standard library.
/// Arrange:  the adapter returned by `create_random_source` for each member of
///   `RANDOM_SOURCE_KINDS` in turn, `build_create_random_source_params`
///   carrying the kind override set to the member, and
///   `build_create_random_source_deps(Default::default())` and
///   `build_create_random_source_payload(Default::default())` as its deps and
///   payload.
/// Act:      `declaration()` on the returned adapter.
/// Assert:   `source` equals the member's `kind`; `adapter_version` equals the
///   member's declared adapter version, the literal 1 written in the assertion
///   for the operating-system member; `interface_version` equals
///   `RANDOM_SOURCE_INTERFACE_VERSION`.
#[test]
fn every_source_from_the_factory_reports_the_declaration_of_its_kind() {
    for kind in RANDOM_SOURCE_KINDS {
        // Arrange
        let kind: RandomSourceKind = *kind;
        let params = build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(kind),
        });
        let adapter = create_random_source(
            &build_create_random_source_deps(Default::default()),
            params,
            build_create_random_source_payload(Default::default()),
        )
        .expect("every member of the declared set constructs its source")
        .adapter;

        // Act
        let declaration = adapter.declaration();

        // Assert
        assert_eq!(declaration.source, kind);
        assert_eq!(declaration.adapter_version, 1);
        assert_eq!(
            declaration.interface_version,
            RANDOM_SOURCE_INTERFACE_VERSION
        );
    }
}
