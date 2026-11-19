#![allow(clippy::expect_used)]

use super::interface::{
    HANDSHAKE_KEY_COMMITMENT_LENGTH, HandshakeKeyCommitment,
    HandshakeKeyCommitmentTryNewErrorReturn,
};
use super::mock::{
    HandshakeKeyCommitmentConstructorParamsOverrides, build_handshake_key_commitment,
    build_handshake_key_commitment_constructor_params,
};
use crate::signature_scheme_identifier::provides::{
    SignatureSchemeIdentifierConstructorParamsOverrides, build_signature_scheme_identifier,
};

/// Contract: every byte of `params.bytes` is `0` →
///   `Err(HandshakeKeyCommitmentTryNewErrorReturn::AllZero)`.
/// Arrange: `build_handshake_key_commitment_constructor_params` overriding the
///   bytes with the array of `HANDSHAKE_KEY_COMMITMENT_LENGTH` zero bytes, the
///   value an unassigned `bytes32` slot reads as; the builder's default array
///   and scheme are admitted values, so the override establishes the refused
///   value.
/// Act:     `HandshakeKeyCommitment::try_new(params)`.
/// Assert:  the result equals `Err(HandshakeKeyCommitmentTryNewErrorReturn::
///   AllZero)` by `assert_eq!`.
#[test]
fn try_new_rejects_the_all_zero_commitment() {
    // Arrange
    let params = build_handshake_key_commitment_constructor_params(
        HandshakeKeyCommitmentConstructorParamsOverrides {
            bytes: Some([0; HANDSHAKE_KEY_COMMITMENT_LENGTH]),
            ..Default::default()
        },
    );

    // Act
    let result = HandshakeKeyCommitment::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(HandshakeKeyCommitmentTryNewErrorReturn::AllZero)
    );
}
// try_new_rejects_the_all_zero_commitment

/// Contract: some byte of `params.bytes` is nonzero →
///   `Ok(HandshakeKeyCommitment { bytes, scheme })`.
/// Arrange: `build_handshake_key_commitment_constructor_params` overriding the
///   bytes with an array of zeros whose last offset holds `0x01`, so a check of
///   the first byte alone fails.
/// Act:     `HandshakeKeyCommitment::try_new(params)`.
/// Assert:  the result is `Ok`; the commitment's `bytes` field equals the array
///   literal of zeros with `0x01` at the last offset.
#[test]
fn try_new_admits_a_commitment_whose_only_nonzero_byte_is_the_last() {
    // Arrange
    let mut bytes = [0; HANDSHAKE_KEY_COMMITMENT_LENGTH];
    bytes[HANDSHAKE_KEY_COMMITMENT_LENGTH - 1] = 0x01;
    let params = build_handshake_key_commitment_constructor_params(
        HandshakeKeyCommitmentConstructorParamsOverrides {
            bytes: Some(bytes),
            ..Default::default()
        },
    );

    // Act
    let commitment = HandshakeKeyCommitment::try_new(params)
        .expect("a commitment whose only nonzero byte is the last is admitted");

    // Assert
    assert_eq!(
        commitment.bytes,
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x01,
        ]
    );
}
// try_new_admits_a_commitment_whose_only_nonzero_byte_is_the_last

/// Contract: some byte of `params.bytes` is nonzero →
///   `Ok(HandshakeKeyCommitment { bytes, scheme })`.
/// Arrange: `build_handshake_key_commitment_constructor_params` overriding the
///   bytes with an array of zeros whose first offset holds `0xFF`, so a check
///   of the last byte alone fails.
/// Act:     `HandshakeKeyCommitment::try_new(params)`.
/// Assert:  the result is `Ok`; the commitment's `bytes` field equals the array
///   literal of zeros with `0xFF` at the first offset.
#[test]
fn try_new_admits_a_commitment_whose_only_nonzero_byte_is_the_first() {
    // Arrange
    let mut bytes = [0; HANDSHAKE_KEY_COMMITMENT_LENGTH];
    bytes[0] = 0xFF;
    let params = build_handshake_key_commitment_constructor_params(
        HandshakeKeyCommitmentConstructorParamsOverrides {
            bytes: Some(bytes),
            ..Default::default()
        },
    );

    // Act
    let commitment = HandshakeKeyCommitment::try_new(params)
        .expect("a commitment whose only nonzero byte is the first is admitted");

    // Assert
    assert_eq!(
        commitment.bytes,
        [
            0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ]
    );
}
// try_new_admits_a_commitment_whose_only_nonzero_byte_is_the_first

/// Contract: some byte of `params.bytes` is nonzero →
///   `Ok(HandshakeKeyCommitment { bytes, scheme })`.
/// Arrange: `build_handshake_key_commitment_constructor_params` overriding the
///   bytes with an array of zeros whose offset 16 holds `0x80`, so a check of
///   the first and last bytes alone fails.
/// Act:     `HandshakeKeyCommitment::try_new(params)`.
/// Assert:  the result is `Ok`; the commitment's `bytes` field equals the array
///   literal of zeros with `0x80` at offset 16.
#[test]
fn try_new_admits_a_commitment_whose_only_nonzero_byte_is_interior() {
    // Arrange
    let mut bytes = [0; HANDSHAKE_KEY_COMMITMENT_LENGTH];
    bytes[16] = 0x80;
    let params = build_handshake_key_commitment_constructor_params(
        HandshakeKeyCommitmentConstructorParamsOverrides {
            bytes: Some(bytes),
            ..Default::default()
        },
    );

    // Act
    let commitment = HandshakeKeyCommitment::try_new(params)
        .expect("a commitment whose only nonzero byte is interior is admitted");

    // Assert
    assert_eq!(
        commitment.bytes,
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ]
    );
}
// try_new_admits_a_commitment_whose_only_nonzero_byte_is_interior

/// Contract: `params.bytes` is nonzero →
///   `Ok(HandshakeKeyCommitment { bytes, scheme })` with `scheme` the params'
///   scheme.
/// Arrange: `build_handshake_key_commitment_constructor_params` overriding the
///   scheme with `build_signature_scheme_identifier` overriding the identifier
///   with `0x5A`, a value that differs from the builder's default scheme, so a
///   constructor that supplies its own scheme fails.
/// Act:     `HandshakeKeyCommitment::try_new(params)`.
/// Assert:  the result is `Ok`; the commitment's `scheme` field, read through
///   its `identifier()`, equals the literal `0x5A`.
#[test]
fn try_new_holds_the_scheme_it_is_handed() {
    // Arrange
    let params = build_handshake_key_commitment_constructor_params(
        HandshakeKeyCommitmentConstructorParamsOverrides {
            scheme: Some(build_signature_scheme_identifier(
                SignatureSchemeIdentifierConstructorParamsOverrides {
                    identifier: Some(0x5A),
                },
            )),
            ..Default::default()
        },
    );

    // Act
    let commitment = HandshakeKeyCommitment::try_new(params)
        .expect("a nonzero commitment is admitted with the scheme it is handed");

    // Assert
    assert_eq!(commitment.scheme.identifier(), 0x5A);
}
// try_new_holds_the_scheme_it_is_handed

/// Contract: `HandshakeKeyCommitment::as_bytes(&self) ->
///   &[u8; HANDSHAKE_KEY_COMMITMENT_LENGTH]` → a shared reference to the held
///   array.
/// Arrange: `build_handshake_key_commitment` overriding the bytes with the
///   array of `0x01` through `0x20` in ascending order, each offset holding a
///   value different from every other offset, so a reversed, rotated, or
///   partial copy fails.
/// Act:     `commitment.as_bytes()`.
/// Assert:  the returned array equals the array literal of `0x01` through
///   `0x20` in ascending order by `assert_eq!`.
#[test]
fn as_bytes_returns_the_held_bytes() {
    // Arrange
    let commitment =
        build_handshake_key_commitment(HandshakeKeyCommitmentConstructorParamsOverrides {
            bytes: Some([
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
                0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C,
                0x1D, 0x1E, 0x1F, 0x20,
            ]),
            ..Default::default()
        });

    // Act
    let bytes = commitment.as_bytes();

    // Assert
    assert_eq!(
        bytes,
        &[
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C,
            0x1D, 0x1E, 0x1F, 0x20,
        ]
    );
}
// as_bytes_returns_the_held_bytes

/// Contract: `HandshakeKeyCommitment::scheme(&self) -> SignatureSchemeIdentifier`
///   → the held scheme.
/// Arrange: `build_handshake_key_commitment` overriding the scheme with
///   `build_signature_scheme_identifier` overriding the identifier with `0x5A`,
///   a value that differs from the builder's default scheme.
/// Act:     `commitment.scheme()`.
/// Assert:  the returned scheme's `identifier()` equals the literal `0x5A` by
///   `assert_eq!`.
#[test]
fn scheme_returns_the_held_scheme() {
    // Arrange
    let commitment =
        build_handshake_key_commitment(HandshakeKeyCommitmentConstructorParamsOverrides {
            scheme: Some(build_signature_scheme_identifier(
                SignatureSchemeIdentifierConstructorParamsOverrides {
                    identifier: Some(0x5A),
                },
            )),
            ..Default::default()
        });

    // Act
    let scheme = commitment.scheme();

    // Assert
    assert_eq!(scheme.identifier(), 0x5A);
}
// scheme_returns_the_held_scheme
