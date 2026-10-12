#![allow(clippy::panic)]

use super::DomainTag;
use super::interface::{DOMAIN_TAG_MAXIMUM_LENGTH, DomainTagTryNewErrorReturn};
use super::mock::{
    DomainTagConstructorParamsOverrides, build_domain_tag, build_domain_tag_constructor_params,
};

/// Contract: `params.bytes.is_empty()` yields `Err(DomainTagTryNewErrorReturn
///   ::Empty)`.
/// Arrange: params whose `bytes` is the empty vector.
/// Act:     `DomainTag::try_new` over those params.
/// Assert:  the result equals `Err(DomainTagTryNewErrorReturn::Empty)` by
///   `assert_eq!`.
#[test]
fn try_new_rejects_an_empty_tag() {
    // Arrange
    let params = build_domain_tag_constructor_params(DomainTagConstructorParamsOverrides {
        bytes: Some(Vec::new()),
    });

    // Act
    let result = DomainTag::try_new(params);

    // Assert
    assert_eq!(result.err(), Some(DomainTagTryNewErrorReturn::Empty));
}

/// Contract: a non-empty tag longer than `DOMAIN_TAG_MAXIMUM_LENGTH` yields
///   `Err(DomainTagTryNewErrorReturn::TooLong { length, maximum:
///   DOMAIN_TAG_MAXIMUM_LENGTH })`.
/// Arrange: params whose `bytes` is `DOMAIN_TAG_MAXIMUM_LENGTH + 1` printable
///   non-space bytes, the least length the check refuses.
/// Act:     `DomainTag::try_new` over those params.
/// Assert:  the result equals `Err(DomainTagTryNewErrorReturn::TooLong
///   { length: DOMAIN_TAG_MAXIMUM_LENGTH + 1, maximum:
///   DOMAIN_TAG_MAXIMUM_LENGTH })` by `assert_eq!`.
#[test]
fn try_new_rejects_a_tag_one_byte_over_the_maximum() {
    // Arrange
    let params = build_domain_tag_constructor_params(DomainTagConstructorParamsOverrides {
        bytes: Some(vec![b'A'; DOMAIN_TAG_MAXIMUM_LENGTH + 1]),
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

/// Contract: a length-passing tag with a byte below `0x20` yields
///   `Err(DomainTagTryNewErrorReturn::ByteOutsidePrintableAscii { index,
///   byte })`.
/// Arrange: params whose `bytes` is three bytes, visible at both edges and
///   `0x1F`, the byte directly below the range, at the interior index.
/// Act:     `DomainTag::try_new` over those params.
/// Assert:  the result equals `Err(DomainTagTryNewErrorReturn
///   ::ByteOutsidePrintableAscii { index: 1, byte: 0x1F })` by `assert_eq!`.
#[test]
fn try_new_rejects_a_byte_below_the_printable_range() {
    // Arrange
    let params = build_domain_tag_constructor_params(DomainTagConstructorParamsOverrides {
        bytes: Some(vec![b'A', 0x1F, b'B']),
    });

    // Act
    let result = DomainTag::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(DomainTagTryNewErrorReturn::ByteOutsidePrintableAscii {
            index: 1,
            byte: 0x1F
        })
    );
}

/// Contract: a length-passing tag with a byte above `0x7E` yields
///   `Err(DomainTagTryNewErrorReturn::ByteOutsidePrintableAscii { index,
///   byte })`.
/// Arrange: params whose `bytes` is three bytes, visible at both edges and
///   `0x7F`, the byte directly above the range, at the interior index.
/// Act:     `DomainTag::try_new` over those params.
/// Assert:  the result equals `Err(DomainTagTryNewErrorReturn
///   ::ByteOutsidePrintableAscii { index: 1, byte: 0x7F })` by `assert_eq!`.
#[test]
fn try_new_rejects_the_byte_above_the_printable_range() {
    // Arrange
    let params = build_domain_tag_constructor_params(DomainTagConstructorParamsOverrides {
        bytes: Some(vec![b'A', 0x7F, b'B']),
    });

    // Act
    let result = DomainTag::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(DomainTagTryNewErrorReturn::ByteOutsidePrintableAscii {
            index: 1,
            byte: 0x7F
        })
    );
}

/// Contract: a length-passing tag with a byte above `0x7F` yields
///   `Err(DomainTagTryNewErrorReturn::ByteOutsidePrintableAscii { index,
///   byte })`.
/// Arrange: params whose `bytes` is three bytes, visible at both edges and
///   `0x80`, the least non-ASCII byte, at the interior index.
/// Act:     `DomainTag::try_new` over those params.
/// Assert:  the result equals `Err(DomainTagTryNewErrorReturn
///   ::ByteOutsidePrintableAscii { index: 1, byte: 0x80 })` by `assert_eq!`.
#[test]
fn try_new_rejects_a_non_ascii_byte() {
    // Arrange
    let params = build_domain_tag_constructor_params(DomainTagConstructorParamsOverrides {
        bytes: Some(vec![b'A', 0x80, b'B']),
    });

    // Act
    let result = DomainTag::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(DomainTagTryNewErrorReturn::ByteOutsidePrintableAscii {
            index: 1,
            byte: 0x80
        })
    );
}

/// Contract: when several bytes are outside printable ASCII, the outcome names
///   the lowest index.
/// Arrange: params whose `bytes` is visible at both edges and holds two
///   offending bytes of different values, `0x1F` at index 1 and `0x80` at
///   index 3, so the lowest index and the later one name different bytes.
/// Act:     `DomainTag::try_new` over those params.
/// Assert:  the result equals `Err(DomainTagTryNewErrorReturn
///   ::ByteOutsidePrintableAscii { index: 1, byte: 0x1F })` by `assert_eq!`.
#[test]
fn try_new_rejects_the_lowest_byte_outside_printable_ascii() {
    // Arrange
    let params = build_domain_tag_constructor_params(DomainTagConstructorParamsOverrides {
        bytes: Some(vec![b'A', 0x1F, b'B', 0x80, b'C']),
    });

    // Act
    let result = DomainTag::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(DomainTagTryNewErrorReturn::ByteOutsidePrintableAscii {
            index: 1,
            byte: 0x1F
        })
    );
}

/// Contract: every byte printable ASCII and `params.bytes.first()` equal to
///   `Some(&b' ')` yields `Err(DomainTagTryNewErrorReturn::SpaceAtEdge
///   { index: 0 })`.
/// Arrange: params whose `bytes` is three bytes, a space then two visible
///   bytes.
/// Act:     `DomainTag::try_new` over those params.
/// Assert:  the result equals `Err(DomainTagTryNewErrorReturn::SpaceAtEdge
///   { index: 0 })` by `assert_eq!`.
#[test]
fn try_new_rejects_a_leading_space() {
    // Arrange
    let params = build_domain_tag_constructor_params(DomainTagConstructorParamsOverrides {
        bytes: Some(vec![b' ', b'A', b'B']),
    });

    // Act
    let result = DomainTag::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(DomainTagTryNewErrorReturn::SpaceAtEdge { index: 0 })
    );
}

/// Contract: a leading byte that is not a space and `params.bytes.last()`
///   equal to `Some(&b' ')` yields `Err(DomainTagTryNewErrorReturn
///   ::SpaceAtEdge { index })`, `index` the length less one.
/// Arrange: params whose `bytes` is three bytes, two visible bytes then a
///   space, so the length less one differs from 0, 1, and the length.
/// Act:     `DomainTag::try_new` over those params.
/// Assert:  the result equals `Err(DomainTagTryNewErrorReturn::SpaceAtEdge
///   { index: 2 })` by `assert_eq!`.
#[test]
fn try_new_rejects_a_trailing_space() {
    // Arrange
    let params = build_domain_tag_constructor_params(DomainTagConstructorParamsOverrides {
        bytes: Some(vec![b'A', b'B', b' ']),
    });

    // Act
    let result = DomainTag::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(DomainTagTryNewErrorReturn::SpaceAtEdge { index: 2 })
    );
}

/// Contract: the leading edge is checked before the trailing edge.
/// Arrange: params whose `bytes` is three bytes, a space, a visible byte, a
///   space, so both edges fail.
/// Act:     `DomainTag::try_new` over those params.
/// Assert:  the result equals `Err(DomainTagTryNewErrorReturn::SpaceAtEdge
///   { index: 0 })` by `assert_eq!`.
#[test]
fn try_new_reports_the_leading_edge_before_the_trailing_edge() {
    // Arrange
    let params = build_domain_tag_constructor_params(DomainTagConstructorParamsOverrides {
        bytes: Some(vec![b' ', b'A', b' ']),
    });

    // Act
    let result = DomainTag::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(DomainTagTryNewErrorReturn::SpaceAtEdge { index: 0 })
    );
}

/// Contract: the byte scan precedes the edge checks.
/// Arrange: params whose `bytes` is a leading space, a visible byte, `0x1F` at
///   index 2, and a visible byte, so an edge check and the scan both fail.
/// Act:     `DomainTag::try_new` over those params.
/// Assert:  the result equals `Err(DomainTagTryNewErrorReturn
///   ::ByteOutsidePrintableAscii { index: 2, byte: 0x1F })` by `assert_eq!`.
#[test]
fn try_new_reports_the_bytes_before_the_edges() {
    // Arrange
    let params = build_domain_tag_constructor_params(DomainTagConstructorParamsOverrides {
        bytes: Some(vec![b' ', b'A', 0x1F, b'B']),
    });

    // Act
    let result = DomainTag::try_new(params);

    // Assert
    assert_eq!(
        result.err(),
        Some(DomainTagTryNewErrorReturn::ByteOutsidePrintableAscii {
            index: 2,
            byte: 0x1F
        })
    );
}

/// Contract: the length check precedes the byte scan.
/// Arrange: params whose `bytes` is `DOMAIN_TAG_MAXIMUM_LENGTH + 1` bytes,
///   printable except `0x1F` at an interior index, so the length check and the
///   scan both fail.
/// Act:     `DomainTag::try_new` over those params.
/// Assert:  the result equals `Err(DomainTagTryNewErrorReturn::TooLong
///   { length: DOMAIN_TAG_MAXIMUM_LENGTH + 1, maximum:
///   DOMAIN_TAG_MAXIMUM_LENGTH })` by `assert_eq!`.
#[test]
fn try_new_reports_the_length_before_the_bytes() {
    // Arrange
    let mut bytes = vec![b'A'; DOMAIN_TAG_MAXIMUM_LENGTH + 1];
    bytes[2] = 0x1F;
    let params = build_domain_tag_constructor_params(DomainTagConstructorParamsOverrides {
        bytes: Some(bytes),
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

/// Contract: every check passing yields `Ok(DomainTag { bytes })`, and a space
///   between visible bytes passes.
/// Arrange: params whose `bytes` is a visible byte, a space, a visible byte.
/// Act:     `DomainTag::try_new` over those params.
/// Assert:  the result mapped to the held `bytes` equals `Ok` of the same
///   three bytes, stated in the assertion, read from the field the test
///   module's visibility reaches.
#[test]
fn try_new_admits_a_tag_with_interior_spaces() {
    // Arrange
    let params = build_domain_tag_constructor_params(DomainTagConstructorParamsOverrides {
        bytes: Some(vec![b'A', b' ', b'B']),
    });

    // Act
    let result = DomainTag::try_new(params);

    // Assert
    assert_eq!(result.map(|tag| tag.bytes), Ok(vec![b'A', b' ', b'B']));
}

/// Contract: a tag of exactly `DOMAIN_TAG_MAXIMUM_LENGTH` bytes passes the
///   length check.
/// Arrange: params whose `bytes` is `DOMAIN_TAG_MAXIMUM_LENGTH` printable
///   non-space bytes, the length directly below the refused length.
/// Act:     `DomainTag::try_new` over those params.
/// Assert:  the result mapped to the held `bytes` equals `Ok` of
///   `DOMAIN_TAG_MAXIMUM_LENGTH` bytes stated in the assertion.
#[test]
fn try_new_admits_a_tag_of_the_maximum_length() {
    // Arrange
    let params = build_domain_tag_constructor_params(DomainTagConstructorParamsOverrides {
        bytes: Some(vec![b'A'; DOMAIN_TAG_MAXIMUM_LENGTH]),
    });

    // Act
    let result = DomainTag::try_new(params);

    // Assert
    assert_eq!(
        result.map(|tag| tag.bytes),
        Ok(vec![b'A'; DOMAIN_TAG_MAXIMUM_LENGTH])
    );
}

/// Contract: `0x21` through `0x7E` are printable and visible, so they pass the
///   scan and the edge checks.
/// Arrange: params whose `bytes` is two bytes, `0x21` first and `0x7E` last.
/// Act:     `DomainTag::try_new` over those params.
/// Assert:  the result mapped to the held `bytes` equals `Ok` of `0x21` then
///   `0x7E`, stated in the assertion.
#[test]
fn try_new_admits_the_lowest_and_highest_graphic_bytes_at_the_edges() {
    // Arrange
    let params = build_domain_tag_constructor_params(DomainTagConstructorParamsOverrides {
        bytes: Some(vec![0x21, 0x7E]),
    });

    // Act
    let result = DomainTag::try_new(params);

    // Assert
    assert_eq!(result.map(|tag| tag.bytes), Ok(vec![0x21, 0x7E]));
}

/// Contract: the admitted outcome moves the vector from the params without
///   copy.
/// Arrange: params from the builder with its default bytes, the address and
///   length of `params.bytes` read before the call.
/// Act:     `DomainTag::try_new` over those params.
/// Assert:  the result is `Ok`, and the held `bytes` has the address and the
///   length read before the call, by `assert_eq!`.
#[test]
fn try_new_moves_the_params_bytes_into_the_tag() {
    // Arrange
    let params = build_domain_tag_constructor_params(Default::default());
    let address = params.bytes.as_ptr();
    let length = params.bytes.len();

    // Act
    let result = DomainTag::try_new(params);

    // Assert
    assert_eq!(
        result
            .as_ref()
            .map(|tag| (tag.bytes.as_ptr(), tag.bytes.len())),
        Ok((address, length))
    );
}

/// Contract: `as_bytes` returns a shared reference to the held bytes, no copy.
/// Arrange: a tag from `build_domain_tag` whose `bytes` override is distinct
///   visible bytes.
/// Act:     `DomainTag::as_bytes` on that tag.
/// Assert:  the returned slice has the address and the length of the tag's
///   held `bytes`, by `assert_eq!`.
#[test]
fn as_bytes_returns_the_held_bytes_without_copy() {
    // Arrange
    let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
        bytes: Some(b"Distinct Bytes 0-9".to_vec()),
    });

    // Act
    let slice = tag.as_bytes();

    // Assert
    assert_eq!(
        (slice.as_ptr(), slice.len()),
        (tag.bytes.as_ptr(), tag.bytes.len())
    );
}
