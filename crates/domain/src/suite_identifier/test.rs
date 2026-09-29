#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{SuiteIdentifier, SuiteIdentifierTryNewErrorReturn};
use super::mock::{
    SuiteIdentifierConstructorParamsOverrides, build_suite_identifier_constructor_params,
};

/// Contract: an identifier with any nonzero byte, the last included, and the
///   lowest nonzero version are admitted and read back unchanged.
/// Arrange: params built with `identifier` holding 31 zero bytes followed by
///   `0x01`, and `version: Some(1)`.
/// Act:     `SuiteIdentifier::try_new(params)`.
/// Assert:  `suite.identifier()` equals an array of 31 zero bytes followed by
///   `0x01`, and `suite.version()` equals `1`.
#[test]
fn try_new_admits_an_identifier_whose_only_nonzero_byte_is_the_last_with_version_one() {
    // Arrange
    let mut identifier = [0u8; 32];
    identifier[31] = 0x01;
    let params =
        build_suite_identifier_constructor_params(SuiteIdentifierConstructorParamsOverrides {
            identifier: Some(identifier),
            version: Some(1),
        });

    // Act
    let Ok(suite) = SuiteIdentifier::try_new(params) else {
        panic!("an identifier with a nonzero byte and a nonzero version is admitted")
    };

    // Assert
    let mut expected = [0u8; 32];
    expected[31] = 0x01;
    assert_eq!(suite.identifier(), &expected);
    assert_eq!(suite.version(), 1);
}

/// Contract: the zero check reads every identifier byte, the first included,
///   and the version admits its full width.
/// Arrange: params built with `identifier` holding `0x80` followed by 31 zero
///   bytes, and `version: Some(u16::MAX)`.
/// Act:     `SuiteIdentifier::try_new(params)`.
/// Assert:  `suite.identifier()` equals an array of `0x80` followed by 31 zero
///   bytes, and `suite.version()` equals `65535`.
#[test]
fn try_new_admits_an_identifier_whose_only_nonzero_byte_is_the_first_with_the_largest_version() {
    // Arrange
    let mut identifier = [0u8; 32];
    identifier[0] = 0x80;
    let params =
        build_suite_identifier_constructor_params(SuiteIdentifierConstructorParamsOverrides {
            identifier: Some(identifier),
            version: Some(u16::MAX),
        });

    // Act
    let Ok(suite) = SuiteIdentifier::try_new(params) else {
        panic!("an identifier with a nonzero byte and a nonzero version is admitted")
    };

    // Assert
    let mut expected = [0u8; 32];
    expected[0] = 0x80;
    assert_eq!(suite.identifier(), &expected);
    assert_eq!(suite.version(), 65535);
}

/// Contract: the all-zero identifier, what an unassigned `bytes32` slot reads
///   as, names no registered suite and is refused.
/// Arrange: params built with `identifier: Some([0u8; 32])`.
/// Act:     `SuiteIdentifier::try_new(params)`.
/// Assert:  `error` equals `SuiteIdentifierTryNewErrorReturn::AllZeroIdentifier`.
#[test]
fn try_new_rejects_an_all_zero_identifier() {
    // Arrange
    let params =
        build_suite_identifier_constructor_params(SuiteIdentifierConstructorParamsOverrides {
            identifier: Some([0u8; 32]),
            ..Default::default()
        });

    // Act
    let Err(error) = SuiteIdentifier::try_new(params) else {
        panic!("the all-zero identifier is refused")
    };

    // Assert
    assert_eq!(error, SuiteIdentifierTryNewErrorReturn::AllZeroIdentifier);
}

/// Contract: the zero version, what an unassigned version slot reads as,
///   names no registered suite and is refused.
/// Arrange: params built with `version: Some(0)`.
/// Act:     `SuiteIdentifier::try_new(params)`.
/// Assert:  `error` equals `SuiteIdentifierTryNewErrorReturn::ZeroVersion`.
#[test]
fn try_new_rejects_a_zero_version() {
    // Arrange
    let params =
        build_suite_identifier_constructor_params(SuiteIdentifierConstructorParamsOverrides {
            version: Some(0),
            ..Default::default()
        });

    // Act
    let Err(error) = SuiteIdentifier::try_new(params) else {
        panic!("the zero version is refused")
    };

    // Assert
    assert_eq!(error, SuiteIdentifierTryNewErrorReturn::ZeroVersion);
}

/// Contract: when both fields fail, the identifier's refusal is returned.
/// Arrange: params built with `identifier: Some([0u8; 32])` and
///   `version: Some(0)`.
/// Act:     `SuiteIdentifier::try_new(params)`.
/// Assert:  `error` equals `SuiteIdentifierTryNewErrorReturn::AllZeroIdentifier`.
#[test]
fn try_new_reports_the_identifier_before_the_version() {
    // Arrange
    let params =
        build_suite_identifier_constructor_params(SuiteIdentifierConstructorParamsOverrides {
            identifier: Some([0u8; 32]),
            version: Some(0),
        });

    // Act
    let Err(error) = SuiteIdentifier::try_new(params) else {
        panic!("an all-zero identifier with a zero version is refused")
    };

    // Assert
    assert_eq!(error, SuiteIdentifierTryNewErrorReturn::AllZeroIdentifier);
}
