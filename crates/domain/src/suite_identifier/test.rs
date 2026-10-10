#![allow(clippy::expect_used)]

use super::provides::{
    SUITE_IDENTIFIER_LENGTH, SuiteIdentifier, SuiteIdentifierConstructorParamsOverrides,
    SuiteIdentifierTryNewErrorReturn, build_suite_identifier,
    build_suite_identifier_constructor_params,
};

/// Contract: every byte of `params.identifier` is `0` →
///   `Err(SuiteIdentifierTryNewErrorReturn::AllZeroIdentifier)`.
/// Arrange: `build_suite_identifier_constructor_params` overriding the identifier with
///   the array of `SUITE_IDENTIFIER_LENGTH` zero bytes, the value an unassigned
///   `bytes32` slot reads as; the version keeps its valid default, so the identifier is
///   the only refusal the params can meet.
/// Act:     `SuiteIdentifier::try_new` on the built params.
/// Assert:  the result equals `Err(SuiteIdentifierTryNewErrorReturn::
///   AllZeroIdentifier)` by `assert_eq!`.
#[test]
fn try_new_rejects_an_all_zero_identifier() {
    // Arrange
    let params =
        build_suite_identifier_constructor_params(SuiteIdentifierConstructorParamsOverrides {
            identifier: Some([0; SUITE_IDENTIFIER_LENGTH]),
            ..Default::default()
        });

    // Act
    let result = SuiteIdentifier::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(SuiteIdentifierTryNewErrorReturn::AllZeroIdentifier)
    );
}

/// Contract: the identifier passes and `params.version` is `0` →
///   `Err(SuiteIdentifierTryNewErrorReturn::ZeroVersion)`.
/// Arrange: `build_suite_identifier_constructor_params` overriding the version with
///   `0`; the identifier keeps its valid nonzero default, so the version is the only
///   refusal the params can meet.
/// Act:     `SuiteIdentifier::try_new` on the built params.
/// Assert:  the result equals `Err(SuiteIdentifierTryNewErrorReturn::ZeroVersion)` by
///   `assert_eq!`.
#[test]
fn try_new_rejects_a_zero_version() {
    // Arrange
    let params =
        build_suite_identifier_constructor_params(SuiteIdentifierConstructorParamsOverrides {
            version: Some(0),
            ..Default::default()
        });

    // Act
    let result = SuiteIdentifier::try_new(params);

    // Assert
    assert_eq!(result, Err(SuiteIdentifierTryNewErrorReturn::ZeroVersion));
}

/// Contract: the identifier's check precedes the version's → an all-zero identifier
///   with a zero version yields `Err(SuiteIdentifierTryNewErrorReturn::
///   AllZeroIdentifier)`.
/// Arrange: `build_suite_identifier_constructor_params` overriding the identifier with
///   the array of `SUITE_IDENTIFIER_LENGTH` zero bytes and the version with `0`, so a
///   version check that runs ahead of the identifier check fails.
/// Act:     `SuiteIdentifier::try_new` on the built params.
/// Assert:  the result equals `Err(SuiteIdentifierTryNewErrorReturn::
///   AllZeroIdentifier)` by `assert_eq!`.
#[test]
fn try_new_reports_the_identifier_before_the_version() {
    // Arrange
    let params =
        build_suite_identifier_constructor_params(SuiteIdentifierConstructorParamsOverrides {
            identifier: Some([0; SUITE_IDENTIFIER_LENGTH]),
            version: Some(0),
        });

    // Act
    let result = SuiteIdentifier::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(SuiteIdentifierTryNewErrorReturn::AllZeroIdentifier)
    );
}

/// Contract: some byte of `params.identifier` is nonzero and `params.version` is
///   nonzero → `Ok(SuiteIdentifier { identifier, version })`.
/// Arrange: `build_suite_identifier_constructor_params` overriding the identifier with
///   an array of zeros whose last offset holds `0x01` and the version with `1`, the
///   smallest admitted version, so a check of the first identifier byte alone fails.
/// Act:     `SuiteIdentifier::try_new` on the built params.
/// Assert:  the result is `Ok`; the suite identifier's `identifier` field equals the
///   array literal of zeros with `0x01` at the last offset and its `version` field
///   equals the literal `1`.
#[test]
fn try_new_admits_an_identifier_whose_only_nonzero_byte_is_the_last_with_version_one() {
    // Arrange
    let mut identifier = [0; SUITE_IDENTIFIER_LENGTH];
    identifier[SUITE_IDENTIFIER_LENGTH - 1] = 0x01;
    let params =
        build_suite_identifier_constructor_params(SuiteIdentifierConstructorParamsOverrides {
            identifier: Some(identifier),
            version: Some(1),
        });

    // Act
    let suite_identifier = SuiteIdentifier::try_new(params)
        .expect("an identifier whose only nonzero byte is the last is admitted");

    // Assert
    assert_eq!(
        suite_identifier.identifier,
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x01,
        ]
    );
    assert_eq!(suite_identifier.version, 1);
}

/// Contract: some byte of `params.identifier` is nonzero and `params.version` is
///   nonzero → `Ok(SuiteIdentifier { identifier, version })`.
/// Arrange: `build_suite_identifier_constructor_params` overriding the identifier with
///   an array of zeros whose first offset holds `0xFF` and the version with `0xFFFF`,
///   the largest `u16`, so a check of the last identifier byte alone fails.
/// Act:     `SuiteIdentifier::try_new` on the built params.
/// Assert:  the result is `Ok`; the suite identifier's `identifier` field equals the
///   array literal of zeros with `0xFF` at the first offset and its `version` field
///   equals the literal `0xFFFF`.
#[test]
fn try_new_admits_an_identifier_whose_only_nonzero_byte_is_the_first_with_the_largest_version() {
    // Arrange
    let mut identifier = [0; SUITE_IDENTIFIER_LENGTH];
    identifier[0] = 0xFF;
    let params =
        build_suite_identifier_constructor_params(SuiteIdentifierConstructorParamsOverrides {
            identifier: Some(identifier),
            version: Some(0xFFFF),
        });

    // Act
    let suite_identifier = SuiteIdentifier::try_new(params)
        .expect("an identifier whose only nonzero byte is the first is admitted");

    // Assert
    assert_eq!(
        suite_identifier.identifier,
        [
            0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ]
    );
    assert_eq!(suite_identifier.version, 0xFFFF);
}

/// Contract: some byte of `params.identifier` is nonzero and `params.version` is
///   nonzero → `Ok(SuiteIdentifier { identifier, version })`.
/// Arrange: `build_suite_identifier_constructor_params` overriding the identifier with
///   an array of zeros whose offset 16 holds `0x80` and the version with `1`, so a
///   check of the first and last identifier bytes alone fails.
/// Act:     `SuiteIdentifier::try_new` on the built params.
/// Assert:  the result is `Ok`; the suite identifier's `identifier` field equals the
///   array literal of zeros with `0x80` at offset 16 and its `version` field equals the
///   literal `1`.
#[test]
fn try_new_admits_an_identifier_whose_only_nonzero_byte_is_interior_with_version_one() {
    // Arrange
    let mut identifier = [0; SUITE_IDENTIFIER_LENGTH];
    identifier[16] = 0x80;
    let params =
        build_suite_identifier_constructor_params(SuiteIdentifierConstructorParamsOverrides {
            identifier: Some(identifier),
            version: Some(1),
        });

    // Act
    let suite_identifier = SuiteIdentifier::try_new(params)
        .expect("an identifier whose only nonzero byte is interior is admitted");

    // Assert
    assert_eq!(
        suite_identifier.identifier,
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ]
    );
    assert_eq!(suite_identifier.version, 1);
}

/// Contract: `SuiteIdentifier::identifier(&self) -> &[u8; SUITE_IDENTIFIER_LENGTH]` →
///   a shared reference to the held array.
/// Arrange: `build_suite_identifier` overriding the identifier with the array of `0x01`
///   through `0x20` in ascending order, each offset holding a value different from
///   every other offset, so a reversed, rotated, or partial copy fails.
/// Act:     `suite_identifier.identifier()`.
/// Assert:  the returned array equals the array literal of `0x01` through `0x20` in
///   ascending order by `assert_eq!`.
#[test]
fn identifier_returns_the_held_identifier() {
    // Arrange
    let suite_identifier = build_suite_identifier(SuiteIdentifierConstructorParamsOverrides {
        identifier: Some([
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C,
            0x1D, 0x1E, 0x1F, 0x20,
        ]),
        ..Default::default()
    });

    // Act
    let identifier = suite_identifier.identifier();

    // Assert
    assert_eq!(
        identifier,
        &[
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C,
            0x1D, 0x1E, 0x1F, 0x20,
        ]
    );
}

/// Contract: `SuiteIdentifier::version(&self) -> u16` → the held version.
/// Arrange: `build_suite_identifier` overriding the version with `0x0102`, a value
///   whose two bytes differ, so a byte-swapped or truncated return fails.
/// Act:     `suite_identifier.version()`.
/// Assert:  the returned `u16` equals the literal `0x0102` by `assert_eq!`.
#[test]
fn version_returns_the_held_version() {
    // Arrange
    let suite_identifier = build_suite_identifier(SuiteIdentifierConstructorParamsOverrides {
        version: Some(0x0102),
        ..Default::default()
    });

    // Act
    let version = suite_identifier.version();

    // Assert
    assert_eq!(version, 0x0102);
}
