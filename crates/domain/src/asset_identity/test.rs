#![allow(clippy::expect_used)]

use super::provides::{
    AssetIdentity, AssetIdentityConstructorParamsOverrides, AssetIdentityTryNewErrorReturn,
    build_asset_coordinate, build_asset_identity, build_asset_identity_constructor_params,
};

/// Contract: every check passes → `Ok(AssetIdentity { name, version })`; a name may
///   contain `@`.
/// Arrange: `build_asset_identity_constructor_params` overriding the name with the
///   scoped name `@scope/pkg` and the version with the prerelease version
///   `1.0.0-beta.1`; the name carries `@` and `/` and the version carries `-` and `.`.
/// Act:     `AssetIdentity::try_new` on the built params.
/// Assert:  the identity's `name` field equals the literal `"@scope/pkg"` and its
///   `version` field equals the literal `"1.0.0-beta.1"`.
#[test]
fn try_new_admits_a_scoped_name_and_a_prerelease_version() {
    // Arrange
    let params = build_asset_identity_constructor_params(AssetIdentityConstructorParamsOverrides {
        name: Some("@scope/pkg".to_string()),
        version: Some("1.0.0-beta.1".to_string()),
    });

    // Act
    let identity = AssetIdentity::try_new(params)
        .expect("a scoped name and a prerelease version are admitted");

    // Assert
    assert_eq!(identity.name, "@scope/pkg");
    assert_eq!(identity.version, "1.0.0-beta.1");
}

/// Contract: every check passes → `Ok(AssetIdentity { name, version })`; visible ASCII
///   is `0x21` through `0x7E`, ends included.
/// Arrange: `build_asset_identity_constructor_params` overriding the name with `!~` and
///   the version with `!~`, the bytes `0x21` and `0x7E` in each string.
/// Act:     `AssetIdentity::try_new` on the built params.
/// Assert:  the identity's `name` field equals the literal `"!~"` and its `version`
///   field equals the literal `"!~"`.
#[test]
fn try_new_admits_the_lowest_and_highest_visible_ascii_bytes() {
    // Arrange
    let params = build_asset_identity_constructor_params(AssetIdentityConstructorParamsOverrides {
        name: Some("!~".to_string()),
        version: Some("!~".to_string()),
    });

    // Act
    let identity = AssetIdentity::try_new(params)
        .expect("the lowest and highest visible ASCII bytes are admitted");

    // Assert
    assert_eq!(identity.name, "!~");
    assert_eq!(identity.version, "!~");
}

/// Contract: `params.name.is_empty()` → `Err(AssetIdentityTryNewErrorReturn::EmptyName)`.
/// Arrange: `build_asset_identity_constructor_params` overriding the name with the
///   empty string; the version keeps its valid default, so the emptiness check is the
///   only refusal the params can meet.
/// Act:     `AssetIdentity::try_new` on the built params.
/// Assert:  the result equals `Err(AssetIdentityTryNewErrorReturn::EmptyName)` by
///   `assert_eq!`.
#[test]
fn try_new_rejects_an_empty_name() {
    // Arrange
    let params = build_asset_identity_constructor_params(AssetIdentityConstructorParamsOverrides {
        name: Some(String::new()),
        ..Default::default()
    });

    // Act
    let result = AssetIdentity::try_new(params);

    // Assert
    assert_eq!(result, Err(AssetIdentityTryNewErrorReturn::EmptyName));
}

/// Contract: the name is non-empty and some byte of `params.name.bytes()` fails
///   `is_ascii_graphic` → `Err(AssetIdentityTryNewErrorReturn::NameByteOutsideVisibleAscii
///   { index, byte })` for the lowest such index.
/// Arrange: `build_asset_identity_constructor_params` overriding the name with `ab cd`
///   followed by a tab byte and `e`, which places a space byte at offset 2 and a tab
///   byte at offset 5, offending bytes of different values at different offsets; the
///   version keeps its valid default.
/// Act:     `AssetIdentity::try_new` on the built params.
/// Assert:  the result equals `Err(AssetIdentityTryNewErrorReturn::
///   NameByteOutsideVisibleAscii { index: 2, byte: 0x20 })` by `assert_eq!`, the offset
///   and value of the lowest offending byte the arrangement places.
#[test]
fn try_new_rejects_the_lowest_name_byte_outside_visible_ascii() {
    // Arrange
    let params = build_asset_identity_constructor_params(AssetIdentityConstructorParamsOverrides {
        name: Some("ab cd\te".to_string()),
        ..Default::default()
    });

    // Act
    let result = AssetIdentity::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(
            AssetIdentityTryNewErrorReturn::NameByteOutsideVisibleAscii {
                index: 2,
                byte: 0x20
            }
        )
    );
}

/// Contract: the name is non-empty and some byte of `params.name.bytes()` fails
///   `is_ascii_graphic` → `Err(AssetIdentityTryNewErrorReturn::NameByteOutsideVisibleAscii
///   { index, byte })`, the error carrying the byte and never the character.
/// Arrange: `build_asset_identity_constructor_params` overriding the name with `aé`, an
///   ASCII byte followed by the character U+00E9, whose UTF-8 bytes `0xC3 0xA9` sit at
///   offsets 1 and 2; the version keeps its valid default.
/// Act:     `AssetIdentity::try_new` on the built params.
/// Assert:  the result equals `Err(AssetIdentityTryNewErrorReturn::
///   NameByteOutsideVisibleAscii { index: 1, byte: 0xC3 })` by `assert_eq!`, the offset
///   and value of the first byte of the encoded character.
#[test]
fn try_new_rejects_a_non_ascii_name() {
    // Arrange
    let params = build_asset_identity_constructor_params(AssetIdentityConstructorParamsOverrides {
        name: Some("aé".to_string()),
        ..Default::default()
    });

    // Act
    let result = AssetIdentity::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(
            AssetIdentityTryNewErrorReturn::NameByteOutsideVisibleAscii {
                index: 1,
                byte: 0xC3
            }
        )
    );
}

/// Contract: the name passes and `params.version.is_empty()` →
///   `Err(AssetIdentityTryNewErrorReturn::EmptyVersion)`.
/// Arrange: `build_asset_identity_constructor_params` overriding the version with the
///   empty string; the name keeps its valid default, so the emptiness check is the only
///   refusal the params can meet.
/// Act:     `AssetIdentity::try_new` on the built params.
/// Assert:  the result equals `Err(AssetIdentityTryNewErrorReturn::EmptyVersion)` by
///   `assert_eq!`.
#[test]
fn try_new_rejects_an_empty_version() {
    // Arrange
    let params = build_asset_identity_constructor_params(AssetIdentityConstructorParamsOverrides {
        version: Some(String::new()),
        ..Default::default()
    });

    // Act
    let result = AssetIdentity::try_new(params);

    // Assert
    assert_eq!(result, Err(AssetIdentityTryNewErrorReturn::EmptyVersion));
}

/// Contract: the name passes, the version is non-empty, and some byte of
///   `params.version.bytes()` fails `is_ascii_graphic` →
///   `Err(AssetIdentityTryNewErrorReturn::VersionByteOutsideVisibleAscii
///   { index, byte })`.
/// Arrange: `build_asset_identity_constructor_params` overriding the version with `1.0`
///   followed by the DEL byte `0x7F`, the byte just above the range, at offset 3; the
///   name keeps its valid default.
/// Act:     `AssetIdentity::try_new` on the built params.
/// Assert:  the result equals `Err(AssetIdentityTryNewErrorReturn::
///   VersionByteOutsideVisibleAscii { index: 3, byte: 0x7F })` by `assert_eq!`, the
///   offset and value of the offending byte the arrangement places.
#[test]
fn try_new_rejects_a_version_byte_outside_visible_ascii() {
    // Arrange
    let params = build_asset_identity_constructor_params(AssetIdentityConstructorParamsOverrides {
        version: Some("1.0\u{7f}".to_string()),
        ..Default::default()
    });

    // Act
    let result = AssetIdentity::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(
            AssetIdentityTryNewErrorReturn::VersionByteOutsideVisibleAscii {
                index: 3,
                byte: 0x7F
            }
        )
    );
}

/// Contract: the name passes, the version is non-empty, and some byte of
///   `params.version.bytes()` fails `is_ascii_graphic` →
///   `Err(AssetIdentityTryNewErrorReturn::VersionByteOutsideVisibleAscii
///   { index, byte })`, the error carrying the byte and never the character.
/// Arrange: `build_asset_identity_constructor_params` overriding the version with `1é`,
///   an ASCII byte followed by the character U+00E9, whose UTF-8 bytes `0xC3 0xA9` sit
///   at offsets 1 and 2; the name keeps its valid default.
/// Act:     `AssetIdentity::try_new` on the built params.
/// Assert:  the result equals `Err(AssetIdentityTryNewErrorReturn::
///   VersionByteOutsideVisibleAscii { index: 1, byte: 0xC3 })` by `assert_eq!`, the
///   offset and value of the first byte of the encoded character.
#[test]
fn try_new_rejects_a_non_ascii_version() {
    // Arrange
    let params = build_asset_identity_constructor_params(AssetIdentityConstructorParamsOverrides {
        version: Some("1é".to_string()),
        ..Default::default()
    });

    // Act
    let result = AssetIdentity::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(
            AssetIdentityTryNewErrorReturn::VersionByteOutsideVisibleAscii {
                index: 1,
                byte: 0xC3
            }
        )
    );
}

/// Contract: the name passes, the version is non-empty, and some byte of
///   `params.version.bytes()` equals `ASSET_COORDINATE_SEPARATOR` →
///   `Err(AssetIdentityTryNewErrorReturn::VersionContainsSeparator { index })`.
/// Arrange: `build_asset_identity_constructor_params` overriding the version with
///   `1.0@0`, the separator at offset 3; the name keeps its valid default.
/// Act:     `AssetIdentity::try_new` on the built params.
/// Assert:  the result equals `Err(AssetIdentityTryNewErrorReturn::
///   VersionContainsSeparator { index: 3 })` by `assert_eq!`, the offset of the
///   separator the arrangement places.
#[test]
fn try_new_rejects_a_version_containing_the_separator() {
    // Arrange
    let params = build_asset_identity_constructor_params(AssetIdentityConstructorParamsOverrides {
        version: Some("1.0@0".to_string()),
        ..Default::default()
    });

    // Act
    let result = AssetIdentity::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(AssetIdentityTryNewErrorReturn::VersionContainsSeparator { index: 3 })
    );
}

/// Contract: the version is scanned once left to right and the first byte that fails
///   `is_ascii_graphic` or equals `ASSET_COORDINATE_SEPARATOR` decides →
///   `Err(AssetIdentityTryNewErrorReturn::VersionContainsSeparator { index })` when that
///   byte is the separator.
/// Arrange: `build_asset_identity_constructor_params` overriding the version with `1@0`
///   followed by the DEL byte `0x7F`, the separator at offset 1 and the DEL byte at
///   offset 3; the name keeps its valid default.
/// Act:     `AssetIdentity::try_new` on the built params.
/// Assert:  the result equals `Err(AssetIdentityTryNewErrorReturn::
///   VersionContainsSeparator { index: 1 })` by `assert_eq!`, the offset of the lowest
///   offending byte the arrangement places.
#[test]
fn try_new_reports_a_version_separator_before_a_later_byte_outside_visible_ascii() {
    // Arrange
    let params = build_asset_identity_constructor_params(AssetIdentityConstructorParamsOverrides {
        version: Some("1@0\u{7f}".to_string()),
        ..Default::default()
    });

    // Act
    let result = AssetIdentity::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(AssetIdentityTryNewErrorReturn::VersionContainsSeparator { index: 1 })
    );
}

/// Contract: the version is scanned once left to right and the first byte that fails
///   `is_ascii_graphic` or equals `ASSET_COORDINATE_SEPARATOR` decides →
///   `Err(AssetIdentityTryNewErrorReturn::VersionByteOutsideVisibleAscii
///   { index, byte })` when that byte fails `is_ascii_graphic`.
/// Arrange: `build_asset_identity_constructor_params` overriding the version with `1`,
///   the DEL byte `0x7F`, `0`, and `@`, the DEL byte at offset 1 and the separator at
///   offset 3; the name keeps its valid default.
/// Act:     `AssetIdentity::try_new` on the built params.
/// Assert:  the result equals `Err(AssetIdentityTryNewErrorReturn::
///   VersionByteOutsideVisibleAscii { index: 1, byte: 0x7F })` by `assert_eq!`, the
///   offset and value of the lowest offending byte the arrangement places.
#[test]
fn try_new_reports_a_version_byte_outside_visible_ascii_before_a_later_separator() {
    // Arrange
    let params = build_asset_identity_constructor_params(AssetIdentityConstructorParamsOverrides {
        version: Some("1\u{7f}0@".to_string()),
        ..Default::default()
    });

    // Act
    let result = AssetIdentity::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(
            AssetIdentityTryNewErrorReturn::VersionByteOutsideVisibleAscii {
                index: 1,
                byte: 0x7F
            }
        )
    );
}

/// Contract: the name's checks precede the version's → a name byte outside visible
///   ASCII with an empty version yields `Err(AssetIdentityTryNewErrorReturn::
///   NameByteOutsideVisibleAscii { index, byte })`.
/// Arrange: `build_asset_identity_constructor_params` overriding the name with `a b`, a
///   space byte at offset 1, and the version with the empty string, so every check of
///   the version that runs ahead of the name's byte check fails.
/// Act:     `AssetIdentity::try_new` on the built params.
/// Assert:  the result equals `Err(AssetIdentityTryNewErrorReturn::
///   NameByteOutsideVisibleAscii { index: 1, byte: 0x20 })` by `assert_eq!`, the offset
///   and value of the offending byte the arrangement places in the name.
#[test]
fn try_new_reports_the_name_before_an_empty_version() {
    // Arrange
    let params = build_asset_identity_constructor_params(AssetIdentityConstructorParamsOverrides {
        name: Some("a b".to_string()),
        version: Some(String::new()),
    });

    // Act
    let result = AssetIdentity::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(
            AssetIdentityTryNewErrorReturn::NameByteOutsideVisibleAscii {
                index: 1,
                byte: 0x20
            }
        )
    );
}

/// Contract: the name's checks precede the version's → a name byte outside visible
///   ASCII with a version containing the separator yields
///   `Err(AssetIdentityTryNewErrorReturn::NameByteOutsideVisibleAscii { index, byte })`.
/// Arrange: `build_asset_identity_constructor_params` overriding the name with `a b`, a
///   space byte at offset 1, and the version with `1@0`, the separator at offset 1, so
///   a scan of the version that runs ahead of the scan of the name fails.
/// Act:     `AssetIdentity::try_new` on the built params.
/// Assert:  the result equals `Err(AssetIdentityTryNewErrorReturn::
///   NameByteOutsideVisibleAscii { index: 1, byte: 0x20 })` by `assert_eq!`, the offset
///   and value of the offending byte the arrangement places in the name.
#[test]
fn try_new_reports_the_name_before_the_version_bytes() {
    // Arrange
    let params = build_asset_identity_constructor_params(AssetIdentityConstructorParamsOverrides {
        name: Some("a b".to_string()),
        version: Some("1@0".to_string()),
    });

    // Act
    let result = AssetIdentity::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(
            AssetIdentityTryNewErrorReturn::NameByteOutsideVisibleAscii {
                index: 1,
                byte: 0x20
            }
        )
    );
}

/// Contract: `AssetIdentity::name(&self) -> &str` → a shared reference to the held name.
/// Arrange: `build_asset_identity` overriding the name with `example-name` and the
///   version with `9.9.9`, distinct strings, so an accessor that returns the version
///   fails.
/// Act:     `identity.name()`.
/// Assert:  the returned `&str` equals the literal `"example-name"` by `assert_eq!`.
#[test]
fn name_returns_the_held_name() {
    // Arrange
    let identity = build_asset_identity(AssetIdentityConstructorParamsOverrides {
        name: Some("example-name".to_string()),
        version: Some("9.9.9".to_string()),
    });

    // Act
    let name = identity.name();

    // Assert
    assert_eq!(name, "example-name");
}

/// Contract: `AssetIdentity::version(&self) -> &str` → a shared reference to the held
///   version.
/// Arrange: `build_asset_identity` overriding the name with `example-name` and the
///   version with `9.9.9`, distinct strings, so an accessor that returns the name fails.
/// Act:     `identity.version()`.
/// Assert:  the returned `&str` equals the literal `"9.9.9"` by `assert_eq!`.
#[test]
fn version_returns_the_held_version() {
    // Arrange
    let identity = build_asset_identity(AssetIdentityConstructorParamsOverrides {
        name: Some("example-name".to_string()),
        version: Some("9.9.9".to_string()),
    });

    // Act
    let version = identity.version();

    // Assert
    assert_eq!(version, "9.9.9");
}

/// Contract: `AssetIdentity::coordinate(&self) -> AssetCoordinate` appends the name's
///   ASCII bytes, `ASSET_COORDINATE_SEPARATOR`, and the version's ASCII bytes in that
///   order, with no alternate join or normalization.
/// Arrange: `build_asset_identity` overriding the name with `@scope/pkg` and the
///   version with `1.0.0-beta.1`; the name carries the separator byte, so a join that
///   splits, drops, or repeats the separator at the name's `@` fails.
/// Act:     `identity.coordinate()`.
/// Assert:  the coordinate's `bytes` field equals the bytes of the literal
///   `@scope/pkg@1.0.0-beta.1` by `assert_eq!`.
#[test]
fn coordinate_writes_the_only_registry_hash_preimage() {
    // Arrange
    let identity = build_asset_identity(AssetIdentityConstructorParamsOverrides {
        name: Some("@scope/pkg".to_string()),
        version: Some("1.0.0-beta.1".to_string()),
    });

    // Act
    let coordinate = identity.coordinate();

    // Assert
    assert_eq!(coordinate.bytes, b"@scope/pkg@1.0.0-beta.1".to_vec());
}

/// Contract: `AssetCoordinate` implements `AsRef<[u8]>` → `as_ref` returns the held
///   coordinate bytes as a shared byte slice.
/// Arrange: `build_asset_coordinate` overriding the name with `left-pad` and the
///   version with `1.3.0`.
/// Act:     `coordinate.as_ref()`.
/// Assert:  the returned slice equals the bytes of the literal `left-pad@1.3.0` by
///   `assert_eq!`.
#[test]
fn as_ref_returns_the_registry_hash_preimage() {
    // Arrange
    let coordinate = build_asset_coordinate(AssetIdentityConstructorParamsOverrides {
        name: Some("left-pad".to_string()),
        version: Some("1.3.0".to_string()),
    });

    // Act
    let preimage = coordinate.as_ref();

    // Assert
    assert_eq!(preimage, b"left-pad@1.3.0");
}
