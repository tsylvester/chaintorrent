#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use domain::{
    HANDSHAKE_KEY_COMMITMENT_LENGTH, HandshakeKeyCommitment,
    HandshakeKeyCommitmentConstructorParamsOverrides, HandshakeKeyCommitmentTryNewErrorReturn,
    SignatureSchemeIdentifierConstructorParamsOverrides,
    build_handshake_key_commitment_constructor_params, build_signature_scheme_identifier,
};

/// Contract: the own entry `the_all_zero_commitment_is_refused_whatever_its_scheme`:
///   `HandshakeKeyCommitment::try_new` with `HANDSHAKE_KEY_COMMITMENT_LENGTH`
///   zero bytes → `Err(HandshakeKeyCommitmentTryNewErrorReturn::AllZero)` for
///   each scheme.
/// Arrange: for each of the schemes `build_signature_scheme_identifier` builds
///   with the identifier overridden to `1` and to `0xFF`,
///   `build_handshake_key_commitment_constructor_params` overriding the bytes
///   with the zero array and the scheme with that scheme; a refusal
///   conditioned on the scheme passes for one and fails for the other.
/// Act:     `HandshakeKeyCommitment::try_new(params)` for each.
/// Assert:  each result equals `Err(HandshakeKeyCommitmentTryNewErrorReturn::
///   AllZero)` by `assert_eq!`.
/// Boundary: the public call `HandshakeKeyCommitment::try_new`; the route
///   `try_new` over the params' bytes.
/// Mocked:  none; the types are the real ones and the builders only construct
///   their fixtures.
#[test]
fn the_all_zero_commitment_is_refused_whatever_its_scheme() {
    for identifier in [1, 0xFF] {
        // Arrange
        let scheme = build_signature_scheme_identifier(
            SignatureSchemeIdentifierConstructorParamsOverrides {
                identifier: Some(identifier),
            },
        );
        let params = build_handshake_key_commitment_constructor_params(
            HandshakeKeyCommitmentConstructorParamsOverrides {
                bytes: Some([0; HANDSHAKE_KEY_COMMITMENT_LENGTH]),
                scheme: Some(scheme),
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
}
// the_all_zero_commitment_is_refused_whatever_its_scheme

/// Contract: the own entry `an_admitted_commitment_reads_back_its_bytes_and_its_scheme`:
///   `HandshakeKeyCommitment::try_new` with nonzero bytes and an admitted
///   scheme → `Ok` whose `as_bytes()` is the bytes and whose
///   `scheme().identifier()` is the scheme's identifier.
/// Arrange: `build_handshake_key_commitment_constructor_params` overriding the
///   bytes with the array of `0x01` through `0x20` in ascending order and the
///   scheme with `build_signature_scheme_identifier` overriding the identifier
///   with `0x5A`, a value that differs from the builder's default scheme; a
///   copy that reorders the bytes or supplies its own scheme fails.
/// Act:     `HandshakeKeyCommitment::try_new(params)`, then `as_bytes()` and
///   `scheme()` on the `Ok` value.
/// Assert:  `as_bytes()` equals the array literal of `0x01` through `0x20` in
///   ascending order by `assert_eq!`; `scheme().identifier()` equals the
///   literal `0x5A` by `assert_eq!`.
/// Boundary: the public call `HandshakeKeyCommitment::try_new` and the
///   accessors the `Ok` value exposes; the route `try_new` moving the params'
///   bytes and scheme into the value.
/// Mocked:  none; the types are the real ones and the builders only construct
///   their fixtures.
#[test]
fn an_admitted_commitment_reads_back_its_bytes_and_its_scheme() {
    // Arrange
    let params = build_handshake_key_commitment_constructor_params(
        HandshakeKeyCommitmentConstructorParamsOverrides {
            bytes: Some([
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
                0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C,
                0x1D, 0x1E, 0x1F, 0x20,
            ]),
            scheme: Some(build_signature_scheme_identifier(
                SignatureSchemeIdentifierConstructorParamsOverrides {
                    identifier: Some(0x5A),
                },
            )),
        },
    );

    // Act
    let commitment = HandshakeKeyCommitment::try_new(params)
        .expect("a nonzero commitment with an admitted scheme is admitted");

    // Assert
    assert_eq!(
        commitment.as_bytes(),
        &[
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C,
            0x1D, 0x1E, 0x1F, 0x20,
        ]
    );
    assert_eq!(commitment.scheme().identifier(), 0x5A);
}
// an_admitted_commitment_reads_back_its_bytes_and_its_scheme
