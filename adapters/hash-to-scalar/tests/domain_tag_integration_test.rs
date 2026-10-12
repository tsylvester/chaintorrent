#![cfg(feature = "mocks")]

use hash_to_scalar::{
    DOMAIN_TAG_MAXIMUM_LENGTH, DomainTag, DomainTagConstructorParamsOverrides,
    DomainTagTryNewErrorReturn, build_domain_tag_constructor_params,
};

/// Contract: the own entry, an admitted tag reads back through the crate
///   root; a tag of exactly `DOMAIN_TAG_MAXIMUM_LENGTH` printable bytes with
///   an interior space and a visible byte at each edge yields `Ok(tag)`
///   whose `as_bytes()` yields exactly the bytes passed.
/// Boundary: the crate-root public call `DomainTag::try_new` and the read
///   `DomainTag::as_bytes`, the route `lib.rs` then `domain_tag::provides`
///   then `try_new` then `as_bytes`.
/// Mocked:   nothing; the route reaches no outer-edge collaborator, and the
///   params come from the official mock under the `mocks` feature.
/// Arrange:  params from `build_domain_tag_constructor_params` whose `bytes`
///   is `DOMAIN_TAG_MAXIMUM_LENGTH` printable bytes with an interior space
///   and visible edge bytes, the variation a read that truncates, trims, or
///   re-encodes fails.
/// Act:      `DomainTag::try_new` over those params, the `Ok` tag read
///   through `as_bytes`.
/// Assert:   the result is `Ok`, and the slice `as_bytes` returns equals the
///   expected bytes stated in the assertion as their own value, since the
///   params are moved into the call.
#[test]
fn a_tag_of_the_maximum_length_reads_back_through_the_crate_root() {
    // Arrange
    let mut bytes = vec![b'x'; DOMAIN_TAG_MAXIMUM_LENGTH];
    bytes[1] = b' ';
    let params = build_domain_tag_constructor_params(DomainTagConstructorParamsOverrides {
        bytes: Some(bytes),
    });

    // Act
    let result = DomainTag::try_new(params);

    // Assert
    let mut expected_bytes = vec![b'x'; DOMAIN_TAG_MAXIMUM_LENGTH];
    expected_bytes[1] = b' ';
    assert_eq!(
        result.map(|tag| tag.as_bytes().to_vec()),
        Ok(expected_bytes)
    );
}

/// Contract: the own entry, a refusal reaches the caller whole and yields no
///   tag; a tag of `DOMAIN_TAG_MAXIMUM_LENGTH + 1` printable bytes yields
///   `Err(DomainTagTryNewErrorReturn::TooLong { length:
///   DOMAIN_TAG_MAXIMUM_LENGTH + 1, maximum: DOMAIN_TAG_MAXIMUM_LENGTH })`.
/// Boundary: the crate-root public call `DomainTag::try_new`, the route
///   `lib.rs` then `domain_tag::provides` then `try_new`.
/// Mocked:   nothing; the route reaches no outer-edge collaborator, and the
///   params come from the official mock under the `mocks` feature.
/// Arrange:  params from `build_domain_tag_constructor_params` whose `bytes`
///   is `DOMAIN_TAG_MAXIMUM_LENGTH + 1` printable non-space bytes, one byte
///   over the length the read-back block admits.
/// Act:      `DomainTag::try_new` over those params.
/// Assert:   the result equals the whole `Err(
///   DomainTagTryNewErrorReturn::TooLong { length:
///   DOMAIN_TAG_MAXIMUM_LENGTH + 1, maximum: DOMAIN_TAG_MAXIMUM_LENGTH })`,
///   matched through the crate-root error type, so no `DomainTag` is held.
#[test]
fn a_tag_one_byte_over_the_maximum_is_refused_whole_through_the_crate_root() {
    // Arrange
    let params = build_domain_tag_constructor_params(DomainTagConstructorParamsOverrides {
        bytes: Some(vec![b'x'; DOMAIN_TAG_MAXIMUM_LENGTH + 1]),
    });

    // Act
    let result = DomainTag::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(DomainTagTryNewErrorReturn::TooLong {
            length: DOMAIN_TAG_MAXIMUM_LENGTH + 1,
            maximum: DOMAIN_TAG_MAXIMUM_LENGTH
        })
    );
}
