#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{AssetIdentity, AssetIdentityTryNewErrorReturn};
use super::mock::{
    AssetIdentityConstructorParamsOverrides, build_asset_identity_constructor_params,
};

/// Contract: a name containing `@` and a version with prerelease and build
///   metadata are admitted and read back unchanged.
/// Arrange: params built with `name: Some("@scope/example-package")` and
///   `version: Some("2.1.0-beta.3+build.7")`.
/// Act:     `AssetIdentity::try_new(params)`.
/// Assert:  `identity.name()` equals "@scope/example-package" and
///   `identity.version()` equals "2.1.0-beta.3+build.7".
#[test]
fn try_new_admits_a_scoped_name_and_a_prerelease_version() {
    // Arrange
    let params = build_asset_identity_constructor_params(AssetIdentityConstructorParamsOverrides {
        name: Some("@scope/example-package".to_string()),
        version: Some("2.1.0-beta.3+build.7".to_string()),
    });

    // Act
    let Ok(identity) = AssetIdentity::try_new(params) else {
        panic!("a scoped name and a prerelease version are admitted")
    };

    // Assert
    assert_eq!(identity.name(), "@scope/example-package");
    assert_eq!(identity.version(), "2.1.0-beta.3+build.7");
}

/// Contract: an empty name is refused.
/// Arrange: params built with `name: Some(String::new())`.
/// Act:     `AssetIdentity::try_new(params)`.
/// Assert:  `error` equals `AssetIdentityTryNewErrorReturn::EmptyName`.
#[test]
fn try_new_rejects_an_empty_name() {
    // Arrange
    let params = build_asset_identity_constructor_params(AssetIdentityConstructorParamsOverrides {
        name: Some(String::new()),
        version: None,
    });

    // Act
    let Err(error) = AssetIdentity::try_new(params) else {
        panic!("an empty name is refused")
    };

    // Assert
    assert_eq!(error, AssetIdentityTryNewErrorReturn::EmptyName);
}

/// Contract: of several offending name bytes, the lowest index is reported.
/// Arrange: params built with `name: Some("example package\tx")`, a space at
///   index 7 and a tab at index 15.
/// Act:     `AssetIdentity::try_new(params)`.
/// Assert:  `error` equals
///   `AssetIdentityTryNewErrorReturn::NameByteOutsideVisibleAscii { index: 7, byte: 0x20 }`.
#[test]
fn try_new_rejects_the_lowest_name_byte_outside_visible_ascii() {
    // Arrange
    let params = build_asset_identity_constructor_params(AssetIdentityConstructorParamsOverrides {
        name: Some("example package\tx".to_string()),
        version: None,
    });

    // Act
    let Err(error) = AssetIdentity::try_new(params) else {
        panic!("a name byte outside visible ASCII is refused")
    };

    // Assert
    assert_eq!(
        error,
        AssetIdentityTryNewErrorReturn::NameByteOutsideVisibleAscii {
            index: 7,
            byte: 0x20
        }
    );
}

/// Contract: a name with a non-ASCII character is refused at its first UTF-8
///   byte.
/// Arrange: params built with `name: Some("exämple")`, whose `ä` encodes as
///   `0xC3 0xA4` at index 2.
/// Act:     `AssetIdentity::try_new(params)`.
/// Assert:  `error` equals
///   `AssetIdentityTryNewErrorReturn::NameByteOutsideVisibleAscii { index: 2, byte: 0xC3 }`.
#[test]
fn try_new_rejects_a_non_ascii_name() {
    // Arrange
    let params = build_asset_identity_constructor_params(AssetIdentityConstructorParamsOverrides {
        name: Some("exämple".to_string()),
        version: None,
    });

    // Act
    let Err(error) = AssetIdentity::try_new(params) else {
        panic!("a non-ASCII name is refused")
    };

    // Assert
    assert_eq!(
        error,
        AssetIdentityTryNewErrorReturn::NameByteOutsideVisibleAscii {
            index: 2,
            byte: 0xC3
        }
    );
}

/// Contract: an empty version is refused.
/// Arrange: params built with `version: Some(String::new())`.
/// Act:     `AssetIdentity::try_new(params)`.
/// Assert:  `error` equals `AssetIdentityTryNewErrorReturn::EmptyVersion`.
#[test]
fn try_new_rejects_an_empty_version() {
    // Arrange
    let params = build_asset_identity_constructor_params(AssetIdentityConstructorParamsOverrides {
        name: None,
        version: Some(String::new()),
    });

    // Act
    let Err(error) = AssetIdentity::try_new(params) else {
        panic!("an empty version is refused")
    };

    // Assert
    assert_eq!(error, AssetIdentityTryNewErrorReturn::EmptyVersion);
}

/// Contract: a version byte outside visible ASCII is refused at its index.
/// Arrange: params built with `version: Some("1.0.0\t")`, a tab at index 5.
/// Act:     `AssetIdentity::try_new(params)`.
/// Assert:  `error` equals
///   `AssetIdentityTryNewErrorReturn::VersionByteOutsideVisibleAscii { index: 5, byte: 0x09 }`.
#[test]
fn try_new_rejects_a_version_byte_outside_visible_ascii() {
    // Arrange
    let params = build_asset_identity_constructor_params(AssetIdentityConstructorParamsOverrides {
        name: None,
        version: Some("1.0.0\t".to_string()),
    });

    // Act
    let Err(error) = AssetIdentity::try_new(params) else {
        panic!("a version byte outside visible ASCII is refused")
    };

    // Assert
    assert_eq!(
        error,
        AssetIdentityTryNewErrorReturn::VersionByteOutsideVisibleAscii {
            index: 5,
            byte: 0x09
        }
    );
}

/// Contract: a version containing the separator `@` is refused, so the join
///   `name@version` splits at its last `@` into exactly one name and one
///   version.
/// Arrange: params built with `version: Some("1.0.0@beta")`, the separator at
///   index 5.
/// Act:     `AssetIdentity::try_new(params)`.
/// Assert:  `error` equals
///   `AssetIdentityTryNewErrorReturn::VersionContainsSeparator { index: 5 }`.
#[test]
fn try_new_rejects_a_version_containing_the_separator() {
    // Arrange
    let params = build_asset_identity_constructor_params(AssetIdentityConstructorParamsOverrides {
        name: None,
        version: Some("1.0.0@beta".to_string()),
    });

    // Act
    let Err(error) = AssetIdentity::try_new(params) else {
        panic!("a version containing the separator is refused")
    };

    // Assert
    assert_eq!(
        error,
        AssetIdentityTryNewErrorReturn::VersionContainsSeparator { index: 5 }
    );
}

/// Contract: the version is scanned once left to right and the first
///   offending byte decides — a separator before a non-visible byte reports
///   the separator.
/// Arrange: params built with `version: Some("1@0 0")`, a separator at
///   index 1 and a space at index 3.
/// Act:     `AssetIdentity::try_new(params)`.
/// Assert:  `error` equals
///   `AssetIdentityTryNewErrorReturn::VersionContainsSeparator { index: 1 }`.
#[test]
fn try_new_reports_the_lowest_offending_version_byte_whatever_its_kind() {
    // Arrange
    let params = build_asset_identity_constructor_params(AssetIdentityConstructorParamsOverrides {
        name: None,
        version: Some("1@0 0".to_string()),
    });

    // Act
    let Err(error) = AssetIdentity::try_new(params) else {
        panic!("a version with an offending byte is refused")
    };

    // Assert
    assert_eq!(
        error,
        AssetIdentityTryNewErrorReturn::VersionContainsSeparator { index: 1 }
    );
}

/// Contract: when both strings fail, the name's refusal is returned.
/// Arrange: params built with `name: Some(String::new())` and
///   `version: Some(String::new())`.
/// Act:     `AssetIdentity::try_new(params)`.
/// Assert:  `error` equals `AssetIdentityTryNewErrorReturn::EmptyName`.
#[test]
fn try_new_reports_the_name_before_the_version() {
    // Arrange
    let params = build_asset_identity_constructor_params(AssetIdentityConstructorParamsOverrides {
        name: Some(String::new()),
        version: Some(String::new()),
    });

    // Act
    let Err(error) = AssetIdentity::try_new(params) else {
        panic!("a name that fails is refused")
    };

    // Assert
    assert_eq!(error, AssetIdentityTryNewErrorReturn::EmptyName);
}
