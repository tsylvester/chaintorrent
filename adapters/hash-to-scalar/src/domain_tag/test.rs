#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{DomainTag, DomainTagTryNewErrorReturn};
use super::mock::{DomainTagConstructorParamsOverrides, build_domain_tag_constructor_params};

/// Contract: a printable-ASCII tag with spaces between visible bytes is
///   admitted and read back unchanged — a space between visible bytes takes
///   the admitted branch.
/// Arrange: bytes `b"ChainTorrent v1 identity"`, differing from the builder's
///   default.
/// Act:     `DomainTag::try_new(params)`.
/// Assert:  `tag.as_bytes()` equals `b"ChainTorrent v1 identity"`.
#[test]
fn try_new_admits_a_tag_with_interior_spaces() {
    // Arrange
    let params = build_domain_tag_constructor_params(DomainTagConstructorParamsOverrides {
        bytes: Some(b"ChainTorrent v1 identity".to_vec()),
    });

    // Act
    let Ok(tag) = DomainTag::try_new(params) else {
        panic!("a tag with interior spaces is admitted")
    };

    // Assert
    assert_eq!(tag.as_bytes(), b"ChainTorrent v1 identity");
}

/// Contract: a tag of exactly 255 bytes fits the one-byte length prefix —
///   the maximum length takes the admitted branch.
/// Arrange: bytes 255 copies of `b'a'`.
/// Act:     `DomainTag::try_new(params)`.
/// Assert:  `tag.as_bytes().len()` equals 255.
#[test]
fn try_new_admits_a_tag_of_the_maximum_length() {
    // Arrange
    let params = build_domain_tag_constructor_params(DomainTagConstructorParamsOverrides {
        bytes: Some(vec![b'a'; 255]),
    });

    // Act
    let Ok(tag) = DomainTag::try_new(params) else {
        panic!("a tag of the maximum length is admitted")
    };

    // Assert
    assert_eq!(tag.as_bytes().len(), 255);
}

/// Contract: an empty tag takes the empty branch.
/// Arrange: bytes an empty vector.
/// Act:     `DomainTag::try_new(params)`.
/// Assert:  the error equals `DomainTagTryNewErrorReturn::Empty`.
#[test]
fn try_new_rejects_an_empty_tag() {
    // Arrange
    let params = build_domain_tag_constructor_params(DomainTagConstructorParamsOverrides {
        bytes: Some(Vec::new()),
    });

    // Act
    let Err(error) = DomainTag::try_new(params) else {
        panic!("an empty tag is refused")
    };

    // Assert
    assert_eq!(error, DomainTagTryNewErrorReturn::Empty);
}

/// Contract: a tag of 256 bytes exceeds the one-byte length prefix — the too
///   long branch reports the tag's length and the maximum.
/// Arrange: bytes 256 copies of `b'a'`.
/// Act:     `DomainTag::try_new(params)`.
/// Assert:  the error equals `DomainTagTryNewErrorReturn::TooLong
///   { length: 256, maximum: 255 }`.
#[test]
fn try_new_rejects_a_tag_one_byte_over_the_maximum() {
    // Arrange
    let params = build_domain_tag_constructor_params(DomainTagConstructorParamsOverrides {
        bytes: Some(vec![b'a'; 256]),
    });

    // Act
    let Err(error) = DomainTag::try_new(params) else {
        panic!("a tag one byte over the maximum is refused")
    };

    // Assert
    assert_eq!(
        error,
        DomainTagTryNewErrorReturn::TooLong {
            length: 256,
            maximum: 255
        }
    );
}

/// Contract: of several bytes outside printable ASCII, the lowest index is
///   reported — the byte scan takes the first offending byte.
/// Arrange: bytes `b"Chain\tTorrent\ntag"`, a tab at index 5 and a newline at
///   index 13.
/// Act:     `DomainTag::try_new(params)`.
/// Assert:  the error equals `DomainTagTryNewErrorReturn
///   ::ByteOutsidePrintableAscii { index: 5, byte: 0x09 }`.
#[test]
fn try_new_rejects_the_lowest_byte_outside_printable_ascii() {
    // Arrange
    let params = build_domain_tag_constructor_params(DomainTagConstructorParamsOverrides {
        bytes: Some(b"Chain\tTorrent\ntag".to_vec()),
    });

    // Act
    let Err(error) = DomainTag::try_new(params) else {
        panic!("a tag holding bytes outside printable ASCII is refused")
    };

    // Assert
    assert_eq!(
        error,
        DomainTagTryNewErrorReturn::ByteOutsidePrintableAscii {
            index: 5,
            byte: 0x09
        }
    );
}

/// Contract: a byte above `0x7E` is outside printable ASCII — the byte scan
///   reports it.
/// Arrange: bytes `b'a'`, `0xC3`, `0xA4`, the first non-ASCII byte at index 1.
/// Act:     `DomainTag::try_new(params)`.
/// Assert:  the error equals `DomainTagTryNewErrorReturn
///   ::ByteOutsidePrintableAscii { index: 1, byte: 0xC3 }`.
#[test]
fn try_new_rejects_a_non_ascii_byte() {
    // Arrange
    let params = build_domain_tag_constructor_params(DomainTagConstructorParamsOverrides {
        bytes: Some(vec![b'a', 0xC3, 0xA4]),
    });

    // Act
    let Err(error) = DomainTag::try_new(params) else {
        panic!("a tag holding a non-ASCII byte is refused")
    };

    // Assert
    assert_eq!(
        error,
        DomainTagTryNewErrorReturn::ByteOutsidePrintableAscii {
            index: 1,
            byte: 0xC3
        }
    );
}

/// Contract: a space as the tag's first byte takes the leading-space branch.
/// Arrange: bytes `b" ChainTorrent"`.
/// Act:     `DomainTag::try_new(params)`.
/// Assert:  the error equals `DomainTagTryNewErrorReturn::SpaceAtEdge
///   { index: 0 }`.
#[test]
fn try_new_rejects_a_leading_space() {
    // Arrange
    let params = build_domain_tag_constructor_params(DomainTagConstructorParamsOverrides {
        bytes: Some(b" ChainTorrent".to_vec()),
    });

    // Act
    let Err(error) = DomainTag::try_new(params) else {
        panic!("a tag with a leading space is refused")
    };

    // Assert
    assert_eq!(error, DomainTagTryNewErrorReturn::SpaceAtEdge { index: 0 });
}

/// Contract: a space as the tag's last byte takes the trailing-space branch,
///   reporting the tag's length less one.
/// Arrange: bytes `b"ChainTorrent "`.
/// Act:     `DomainTag::try_new(params)`.
/// Assert:  the error equals `DomainTagTryNewErrorReturn::SpaceAtEdge
///   { index: 12 }`.
#[test]
fn try_new_rejects_a_trailing_space() {
    // Arrange
    let params = build_domain_tag_constructor_params(DomainTagConstructorParamsOverrides {
        bytes: Some(b"ChainTorrent ".to_vec()),
    });

    // Act
    let Err(error) = DomainTag::try_new(params) else {
        panic!("a tag with a trailing space is refused")
    };

    // Assert
    assert_eq!(error, DomainTagTryNewErrorReturn::SpaceAtEdge { index: 12 });
}

/// Contract: a tag with a space at both edges returns the leading refusal —
///   the leading edge is checked before the trailing edge.
/// Arrange: bytes `b" ChainTorrent "`, a space at index 0 and at index 13.
/// Act:     `DomainTag::try_new(params)`.
/// Assert:  the error equals `DomainTagTryNewErrorReturn::SpaceAtEdge
///   { index: 0 }`.
#[test]
fn try_new_reports_the_leading_edge_before_the_trailing_edge() {
    // Arrange
    let params = build_domain_tag_constructor_params(DomainTagConstructorParamsOverrides {
        bytes: Some(b" ChainTorrent ".to_vec()),
    });

    // Act
    let Err(error) = DomainTag::try_new(params) else {
        panic!("a tag with a space at both edges is refused")
    };

    // Assert
    assert_eq!(error, DomainTagTryNewErrorReturn::SpaceAtEdge { index: 0 });
}

/// Contract: a tag that fails both the byte scan and an edge check returns
///   the byte refusal — the byte scan precedes the edge checks.
/// Arrange: bytes `b" Chain\tTorrent"`, a leading space and a tab at index 6.
/// Act:     `DomainTag::try_new(params)`.
/// Assert:  the error equals `DomainTagTryNewErrorReturn
///   ::ByteOutsidePrintableAscii { index: 6, byte: 0x09 }`.
#[test]
fn try_new_reports_the_bytes_before_the_edges() {
    // Arrange
    let params = build_domain_tag_constructor_params(DomainTagConstructorParamsOverrides {
        bytes: Some(b" Chain\tTorrent".to_vec()),
    });

    // Act
    let Err(error) = DomainTag::try_new(params) else {
        panic!("a tag failing the byte scan and an edge check is refused")
    };

    // Assert
    assert_eq!(
        error,
        DomainTagTryNewErrorReturn::ByteOutsidePrintableAscii {
            index: 6,
            byte: 0x09
        }
    );
}

/// Contract: a tag that fails both the length and the byte scan returns the
///   length refusal — the length checks precede the byte scan.
/// Arrange: bytes 256 copies of `0x09`, over the maximum and unprintable.
/// Act:     `DomainTag::try_new(params)`.
/// Assert:  the error equals `DomainTagTryNewErrorReturn::TooLong
///   { length: 256, maximum: 255 }`.
#[test]
fn try_new_reports_the_length_before_the_bytes() {
    // Arrange
    let params = build_domain_tag_constructor_params(DomainTagConstructorParamsOverrides {
        bytes: Some(vec![0x09; 256]),
    });

    // Act
    let Err(error) = DomainTag::try_new(params) else {
        panic!("a tag failing the length and the byte scan is refused")
    };

    // Assert
    assert_eq!(
        error,
        DomainTagTryNewErrorReturn::TooLong {
            length: 256,
            maximum: 255
        }
    );
}
