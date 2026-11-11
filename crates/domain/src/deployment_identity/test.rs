#![allow(clippy::expect_used)]

use super::provides::{
    DEPLOYMENT_IDENTITY_LENGTH, DeploymentIdentity, DeploymentIdentityConstructorParamsOverrides,
    DeploymentIdentityTryNewErrorReturn, build_deployment_identity,
    build_deployment_identity_constructor_params,
};

/// Contract: every byte of `params.bytes` is `0` →
///   `Err(DeploymentIdentityTryNewErrorReturn::AllZero)`.
/// Arrange: `build_deployment_identity_constructor_params` overriding the bytes with
///   the array of `DEPLOYMENT_IDENTITY_LENGTH` zero bytes, the value an unassigned
///   `bytes32` slot reads as; the builder's default array is nonzero, so the override
///   establishes the refused value.
/// Act:     `DeploymentIdentity::try_new` on the built params.
/// Assert:  the result equals `Err(DeploymentIdentityTryNewErrorReturn::AllZero)` by
///   `assert_eq!`.
#[test]
fn try_new_rejects_the_all_zero_identity() {
    // Arrange
    let params = build_deployment_identity_constructor_params(
        DeploymentIdentityConstructorParamsOverrides {
            bytes: Some([0; DEPLOYMENT_IDENTITY_LENGTH]),
        },
    );

    // Act
    let result = DeploymentIdentity::try_new(params);

    // Assert
    assert_eq!(result, Err(DeploymentIdentityTryNewErrorReturn::AllZero));
}

/// Contract: some byte of `params.bytes` is nonzero → `Ok(DeploymentIdentity { bytes })`,
///   the array moved from the params.
/// Arrange: `build_deployment_identity_constructor_params` overriding the bytes with an
///   array of zeros whose last offset holds `0x01`, so a check of the first byte alone
///   fails.
/// Act:     `DeploymentIdentity::try_new` on the built params.
/// Assert:  the result is `Ok`; the identity's `bytes` field equals the array literal of
///   zeros with `0x01` at the last offset.
#[test]
fn try_new_admits_an_identity_whose_only_nonzero_byte_is_the_last() {
    // Arrange
    let mut bytes = [0; DEPLOYMENT_IDENTITY_LENGTH];
    bytes[DEPLOYMENT_IDENTITY_LENGTH - 1] = 0x01;
    let params = build_deployment_identity_constructor_params(
        DeploymentIdentityConstructorParamsOverrides { bytes: Some(bytes) },
    );

    // Act
    let identity = DeploymentIdentity::try_new(params)
        .expect("an identity whose only nonzero byte is the last is admitted");

    // Assert
    assert_eq!(
        identity.bytes,
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x01,
        ]
    );
}

/// Contract: some byte of `params.bytes` is nonzero → `Ok(DeploymentIdentity { bytes })`,
///   the array moved from the params.
/// Arrange: `build_deployment_identity_constructor_params` overriding the bytes with an
///   array of zeros whose first offset holds `0xFF`, so a check of the last byte alone
///   fails.
/// Act:     `DeploymentIdentity::try_new` on the built params.
/// Assert:  the result is `Ok`; the identity's `bytes` field equals the array literal of
///   zeros with `0xFF` at the first offset.
#[test]
fn try_new_admits_an_identity_whose_only_nonzero_byte_is_the_first() {
    // Arrange
    let mut bytes = [0; DEPLOYMENT_IDENTITY_LENGTH];
    bytes[0] = 0xFF;
    let params = build_deployment_identity_constructor_params(
        DeploymentIdentityConstructorParamsOverrides { bytes: Some(bytes) },
    );

    // Act
    let identity = DeploymentIdentity::try_new(params)
        .expect("an identity whose only nonzero byte is the first is admitted");

    // Assert
    assert_eq!(
        identity.bytes,
        [
            0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ]
    );
}

/// Contract: some byte of `params.bytes` is nonzero → `Ok(DeploymentIdentity { bytes })`,
///   the array moved from the params.
/// Arrange: `build_deployment_identity_constructor_params` overriding the bytes with an
///   array of zeros whose offset 16 holds `0x80`, so a check of the first and last
///   bytes alone fails.
/// Act:     `DeploymentIdentity::try_new` on the built params.
/// Assert:  the result is `Ok`; the identity's `bytes` field equals the array literal of
///   zeros with `0x80` at offset 16.
#[test]
fn try_new_admits_an_identity_whose_only_nonzero_byte_is_interior() {
    // Arrange
    let mut bytes = [0; DEPLOYMENT_IDENTITY_LENGTH];
    bytes[16] = 0x80;
    let params = build_deployment_identity_constructor_params(
        DeploymentIdentityConstructorParamsOverrides { bytes: Some(bytes) },
    );

    // Act
    let identity = DeploymentIdentity::try_new(params)
        .expect("an identity whose only nonzero byte is interior is admitted");

    // Assert
    assert_eq!(
        identity.bytes,
        [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ]
    );
}

/// Contract: `DeploymentIdentity::as_bytes(&self) -> &[u8; DEPLOYMENT_IDENTITY_LENGTH]`
///   → a shared reference to the held array.
/// Arrange: `build_deployment_identity` overriding the bytes with the array of `0x01`
///   through `0x20` in ascending order, each offset holding a value different from
///   every other offset, so a reversed, rotated, or partial copy fails.
/// Act:     `identity.as_bytes()`.
/// Assert:  the returned array equals the array literal of `0x01` through `0x20` in
///   ascending order by `assert_eq!`.
#[test]
fn as_bytes_returns_the_held_bytes() {
    // Arrange
    let identity = build_deployment_identity(DeploymentIdentityConstructorParamsOverrides {
        bytes: Some([
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C,
            0x1D, 0x1E, 0x1F, 0x20,
        ]),
    });

    // Act
    let bytes = identity.as_bytes();

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
