#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{ParameterSetIdentifier, ParameterSetIdentifierTryNewErrorReturn};
use super::mock::{
    ParameterSetIdentifierConstructorParamsOverrides,
    build_parameter_set_identifier_constructor_params,
};

/// Contract: a value with any nonzero byte is admitted and read back
///   unchanged, the last byte included.
/// Arrange: params built with `bytes` holding 31 zero bytes followed by `0x01`.
/// Act:     `ParameterSetIdentifier::try_new(params)`.
/// Assert:  `identifier.as_bytes()` equals an array of 31 zero bytes followed
///   by `0x01`.
#[test]
fn try_new_admits_an_identifier_whose_only_nonzero_byte_is_the_last() {
    // Arrange
    let mut bytes = [0u8; 32];
    bytes[31] = 0x01;
    let params = build_parameter_set_identifier_constructor_params(
        ParameterSetIdentifierConstructorParamsOverrides { bytes: Some(bytes) },
    );

    // Act
    let Ok(identifier) = ParameterSetIdentifier::try_new(params) else {
        panic!("an identifier with a nonzero byte is admitted")
    };

    // Assert
    let mut expected = [0u8; 32];
    expected[31] = 0x01;
    assert_eq!(identifier.as_bytes(), &expected);
}

/// Contract: the zero check reads every byte, the first included.
/// Arrange: params built with `bytes` holding `0x80` followed by 31 zero bytes.
/// Act:     `ParameterSetIdentifier::try_new(params)`.
/// Assert:  `identifier.as_bytes()` equals an array of `0x80` followed by 31
///   zero bytes.
#[test]
fn try_new_admits_an_identifier_whose_only_nonzero_byte_is_the_first() {
    // Arrange
    let mut bytes = [0u8; 32];
    bytes[0] = 0x80;
    let params = build_parameter_set_identifier_constructor_params(
        ParameterSetIdentifierConstructorParamsOverrides { bytes: Some(bytes) },
    );

    // Act
    let Ok(identifier) = ParameterSetIdentifier::try_new(params) else {
        panic!("an identifier with a nonzero byte is admitted")
    };

    // Assert
    let mut expected = [0u8; 32];
    expected[0] = 0x80;
    assert_eq!(identifier.as_bytes(), &expected);
}

/// Contract: the all-zero identifier, what an unassigned `bytes32` slot reads
///   as, names no set and is refused.
/// Arrange: params built with `bytes: Some([0u8; 32])`.
/// Act:     `ParameterSetIdentifier::try_new(params)`.
/// Assert:  `error` equals `ParameterSetIdentifierTryNewErrorReturn::AllZero`.
#[test]
fn try_new_rejects_the_all_zero_identifier() {
    // Arrange
    let params = build_parameter_set_identifier_constructor_params(
        ParameterSetIdentifierConstructorParamsOverrides {
            bytes: Some([0u8; 32]),
        },
    );

    // Act
    let Err(error) = ParameterSetIdentifier::try_new(params) else {
        panic!("the all-zero identifier is refused")
    };

    // Assert
    assert_eq!(error, ParameterSetIdentifierTryNewErrorReturn::AllZero);
}
