#![allow(clippy::expect_used)]

use super::provides::{
    SignatureSchemeIdentifier, SignatureSchemeIdentifierConstructorParamsOverrides,
    SignatureSchemeIdentifierTryNewErrorReturn, build_signature_scheme_identifier,
    build_signature_scheme_identifier_constructor_params,
};

/// Contract: `params.identifier` is `0` →
///   `Err(SignatureSchemeIdentifierTryNewErrorReturn::ZeroIdentifier)`.
/// Arrange: `build_signature_scheme_identifier_constructor_params` overriding
///   the identifier with `0`, the value an unassigned `uint8` slot reads as; the
///   builder's default is nonzero, so the override establishes the refused
///   value.
/// Act:     `SignatureSchemeIdentifier::try_new` on the built params.
/// Assert:  the result equals
///   `Err(SignatureSchemeIdentifierTryNewErrorReturn::ZeroIdentifier)` by
///   `assert_eq!`.
#[test]
fn try_new_rejects_a_zero_identifier() {
    // Arrange
    let params = build_signature_scheme_identifier_constructor_params(
        SignatureSchemeIdentifierConstructorParamsOverrides {
            identifier: Some(0),
        },
    );

    // Act
    let result = SignatureSchemeIdentifier::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(SignatureSchemeIdentifierTryNewErrorReturn::ZeroIdentifier)
    );
}
// try_new_rejects_a_zero_identifier

/// Contract: `params.identifier` is nonzero →
///   `Ok(SignatureSchemeIdentifier { identifier })`.
/// Arrange: `build_signature_scheme_identifier_constructor_params` overriding
///   the identifier with `1`, the smallest nonzero `u8`, so a check that
///   refuses below a higher floor fails.
/// Act:     `SignatureSchemeIdentifier::try_new` on the built params.
/// Assert:  the result is `Ok`; its `identifier` field equals the literal `1`.
#[test]
fn try_new_admits_the_smallest_nonzero_identifier() {
    // Arrange
    let params = build_signature_scheme_identifier_constructor_params(
        SignatureSchemeIdentifierConstructorParamsOverrides {
            identifier: Some(1),
        },
    );

    // Act
    let scheme = SignatureSchemeIdentifier::try_new(params)
        .expect("the smallest nonzero identifier is admitted");

    // Assert
    assert_eq!(scheme.identifier, 1);
}
// try_new_admits_the_smallest_nonzero_identifier

/// Contract: `params.identifier` is nonzero →
///   `Ok(SignatureSchemeIdentifier { identifier })`.
/// Arrange: `build_signature_scheme_identifier_constructor_params` overriding
///   the identifier with `0xFF`, the largest `u8`, so a check that caps the
///   value or treats it as signed fails.
/// Act:     `SignatureSchemeIdentifier::try_new` on the built params.
/// Assert:  the result is `Ok`; its `identifier` field equals the literal
///   `0xFF`.
#[test]
fn try_new_admits_the_largest_identifier() {
    // Arrange
    let params = build_signature_scheme_identifier_constructor_params(
        SignatureSchemeIdentifierConstructorParamsOverrides {
            identifier: Some(0xFF),
        },
    );

    // Act
    let scheme =
        SignatureSchemeIdentifier::try_new(params).expect("the largest identifier is admitted");

    // Assert
    assert_eq!(scheme.identifier, 0xFF);
}
// try_new_admits_the_largest_identifier

/// Contract: `SignatureSchemeIdentifier::identifier(&self) -> u8` → the held
///   value.
/// Arrange: `build_signature_scheme_identifier` overriding the identifier with
///   `0x5A`, a value whose bit pattern differs from its complement and its
///   reversal.
/// Act:     `scheme.identifier()`.
/// Assert:  the returned `u8` equals the literal `0x5A` by `assert_eq!`.
#[test]
fn identifier_returns_the_held_identifier() {
    // Arrange
    let scheme =
        build_signature_scheme_identifier(SignatureSchemeIdentifierConstructorParamsOverrides {
            identifier: Some(0x5A),
        });

    // Act
    let identifier = scheme.identifier();

    // Assert
    assert_eq!(identifier, 0x5A);
}
// identifier_returns_the_held_identifier
