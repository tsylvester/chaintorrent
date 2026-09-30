#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{Blake3KeyedKdf, Blake3KeyedKdfConstructorParams};
use crate::factory::provides::{
    DerivationPurpose, DeriveKeyParamsOverrides, DeriveKeyPayload, IKeyDerivationAdapter,
    KDF_INTERFACE_VERSION, KdfIdentifier, build_derive_key_params,
};
use domain::{SecretConstructorParamsOverrides, build_secret};
use hex::decode;

const REFERENCE_CONTEXT_HEX: &str = concat!(
    "0000000000000000000000000000000000000000000000000000000000000140",
    "0000000000000000000000000000000000000000000000000000000000000180",
    "0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a",
    "0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b",
    "0000000000000000000000000000000000000000000000000000000000000003",
    "0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c",
    "0000000000000000000000000000000000000000000000000000000000000005",
    "0000000000000000000000000000000000000000000000000000000000004000",
    "0000000000000000000000000000000000000000000000000000000000008000",
    "0000000000000000000000000000000000000000000000000000000000100000",
    "0000000000000000000000000000000000000000000000000000000000000016",
    "4073636f70652f6578616d706c652d7061636b61676500000000000000000000",
    "000000000000000000000000000000000000000000000000000000000000000c",
    "322e312e302d626574612e330000000000000000000000000000000000000000",
);

const WRAPPING_KEY_VECTOR_HEX: &str = "549874404f0b25b65da55dbce0b410d0aa34085695af4b91303e2c1c0410f760a75cb805271fbaf45eb7a1fa33596b3cb082ec41f923d4fb7055ba82699ec4f3";
const PUBLISHER_ROOT_VECTOR_HEX: &str = "63da685dcbcd9267e8c95361ece3aa6eff7eade6434f8087595e912d41c5abc0d11d8303de6cfa4401b1f192f5ffef30a68fead745af576967b85622951acc58";
const ASSET_ROOT_VECTOR_HEX: &str = "a544b18f012d94c3c1f29cf96563e97b8b6fec6bd076d228a6967e901b4c6ed7257efd1bce6216a1cfd567d2bb8cceb5d67b2fe268777020cb8c5a34bd752801";
const MASTER_SCALAR_VECTOR_HEX: &str = "453eae1adf266f0b865dad65a3adbbc74fd09df871842cd11ca8da7ea896d82d3b96ee39c0d01eeac7b75fde8b07353f73230ec3f21f75439bfb29b72c49cd74";
const IDENTITY_BASES_VECTOR_HEX: &str = "5a2257c26fb7b1e04621baaa23b8a360757b5de336b3aeec6d6562b6d815d721b0e05eec78d8338aad18a60f4eed88459601eed0d17cb6fc54a72a8665bb3864";
const CAPSULE_RANDOMNESS_VECTOR_HEX: &str = "83fea48ca2ce75442b981ab2d9ea1260abadaf7118e022d9e32ea621146a0b2639f3a3cf913284fb6835f1d07aab04fbd483111a2e91c4af6b4991e7faa1966f";
const PIECE_GROUP_KEY_VECTOR_HEX: &str = "f33f881d8d69557235873979f03f0d3e10479c14ddac800785507536cffacd6b893a812db4e36d41ba907b62d7dd1dec19a4c2cff0cbd47218ee6495d1a09e1f";
const PLAINTEXT_ROOT_KEY_VECTOR_HEX: &str = "d8e66fc2126c63ded90ee602192d85f9f1485e2e13b9a310d18846b02aa7e022dfbe765c77fa87014c9b15086629c23609d9968e5e0e3cb5cb5fbae4f047b404";

/// Contract: the wrapping-key purpose's context string, the serialization,
///   and the output match the independent implementation.
/// Arrange: the reference key material and the reference context.
/// Act:     `kdf.derive_key(params, payload)` for `WrappingKey`, length 64.
/// Assert:  `success.key.expose()` equals the wrapping-key vector.
#[test]
fn derive_key_for_the_wrapping_key_matches_the_independent_vector() {
    // Arrange
    let Ok(kdf) = Blake3KeyedKdf::try_new(Blake3KeyedKdfConstructorParams);
    let secret = build_secret(SecretConstructorParamsOverrides {
        value: Some(vec![0x42u8; 32]),
    });
    let Ok(context) = decode(REFERENCE_CONTEXT_HEX) else {
        panic!("the reference context is valid hex")
    };
    let params = build_derive_key_params(DeriveKeyParamsOverrides {
        purpose: Some(DerivationPurpose::WrappingKey),
        length: Some(64),
    });
    let payload = DeriveKeyPayload {
        key_material: &secret,
        context: &context,
    };
    let Ok(expected) = decode(WRAPPING_KEY_VECTOR_HEX) else {
        panic!("the vector is valid hex")
    };

    // Act
    let Ok(success) = kdf.derive_key(params, payload) else {
        panic!("the reference derivation succeeds")
    };

    // Assert
    assert_eq!(success.key.expose(), &expected);
}

/// Contract: the publisher-root purpose's context string, the serialization,
///   and the output match the independent implementation.
/// Arrange: the reference key material and the reference context.
/// Act:     `kdf.derive_key(params, payload)` for `PublisherRoot`, length 64.
/// Assert:  `success.key.expose()` equals the publisher-root vector.
#[test]
fn derive_key_for_the_publisher_root_matches_the_independent_vector() {
    // Arrange
    let Ok(kdf) = Blake3KeyedKdf::try_new(Blake3KeyedKdfConstructorParams);
    let secret = build_secret(SecretConstructorParamsOverrides {
        value: Some(vec![0x42u8; 32]),
    });
    let Ok(context) = decode(REFERENCE_CONTEXT_HEX) else {
        panic!("the reference context is valid hex")
    };
    let params = build_derive_key_params(DeriveKeyParamsOverrides {
        purpose: Some(DerivationPurpose::PublisherRoot),
        length: Some(64),
    });
    let payload = DeriveKeyPayload {
        key_material: &secret,
        context: &context,
    };
    let Ok(expected) = decode(PUBLISHER_ROOT_VECTOR_HEX) else {
        panic!("the vector is valid hex")
    };

    // Act
    let Ok(success) = kdf.derive_key(params, payload) else {
        panic!("the reference derivation succeeds")
    };

    // Assert
    assert_eq!(success.key.expose(), &expected);
}

/// Contract: the asset-root purpose's context string, the serialization,
///   and the output match the independent implementation.
/// Arrange: the reference key material and the reference context.
/// Act:     `kdf.derive_key(params, payload)` for `AssetRoot`, length 64.
/// Assert:  `success.key.expose()` equals the asset-root vector.
#[test]
fn derive_key_for_the_asset_root_matches_the_independent_vector() {
    // Arrange
    let Ok(kdf) = Blake3KeyedKdf::try_new(Blake3KeyedKdfConstructorParams);
    let secret = build_secret(SecretConstructorParamsOverrides {
        value: Some(vec![0x42u8; 32]),
    });
    let Ok(context) = decode(REFERENCE_CONTEXT_HEX) else {
        panic!("the reference context is valid hex")
    };
    let params = build_derive_key_params(DeriveKeyParamsOverrides {
        purpose: Some(DerivationPurpose::AssetRoot),
        length: Some(64),
    });
    let payload = DeriveKeyPayload {
        key_material: &secret,
        context: &context,
    };
    let Ok(expected) = decode(ASSET_ROOT_VECTOR_HEX) else {
        panic!("the vector is valid hex")
    };

    // Act
    let Ok(success) = kdf.derive_key(params, payload) else {
        panic!("the reference derivation succeeds")
    };

    // Assert
    assert_eq!(success.key.expose(), &expected);
}

/// Contract: the master-scalar purpose's context string, the serialization,
///   and the output match the independent implementation.
/// Arrange: the reference key material and the reference context.
/// Act:     `kdf.derive_key(params, payload)` for `MasterScalar`, length 64.
/// Assert:  `success.key.expose()` equals the master-scalar vector.
#[test]
fn derive_key_for_the_master_scalar_matches_the_independent_vector() {
    // Arrange
    let Ok(kdf) = Blake3KeyedKdf::try_new(Blake3KeyedKdfConstructorParams);
    let secret = build_secret(SecretConstructorParamsOverrides {
        value: Some(vec![0x42u8; 32]),
    });
    let Ok(context) = decode(REFERENCE_CONTEXT_HEX) else {
        panic!("the reference context is valid hex")
    };
    let params = build_derive_key_params(DeriveKeyParamsOverrides {
        purpose: Some(DerivationPurpose::MasterScalar),
        length: Some(64),
    });
    let payload = DeriveKeyPayload {
        key_material: &secret,
        context: &context,
    };
    let Ok(expected) = decode(MASTER_SCALAR_VECTOR_HEX) else {
        panic!("the vector is valid hex")
    };

    // Act
    let Ok(success) = kdf.derive_key(params, payload) else {
        panic!("the reference derivation succeeds")
    };

    // Assert
    assert_eq!(success.key.expose(), &expected);
}

/// Contract: the identity-bases purpose's context string, the serialization,
///   and the output match the independent implementation.
/// Arrange: the reference key material and the reference context.
/// Act:     `kdf.derive_key(params, payload)` for `IdentityBases`, length 64.
/// Assert:  `success.key.expose()` equals the identity-bases vector.
#[test]
fn derive_key_for_the_identity_bases_matches_the_independent_vector() {
    // Arrange
    let Ok(kdf) = Blake3KeyedKdf::try_new(Blake3KeyedKdfConstructorParams);
    let secret = build_secret(SecretConstructorParamsOverrides {
        value: Some(vec![0x42u8; 32]),
    });
    let Ok(context) = decode(REFERENCE_CONTEXT_HEX) else {
        panic!("the reference context is valid hex")
    };
    let params = build_derive_key_params(DeriveKeyParamsOverrides {
        purpose: Some(DerivationPurpose::IdentityBases),
        length: Some(64),
    });
    let payload = DeriveKeyPayload {
        key_material: &secret,
        context: &context,
    };
    let Ok(expected) = decode(IDENTITY_BASES_VECTOR_HEX) else {
        panic!("the vector is valid hex")
    };

    // Act
    let Ok(success) = kdf.derive_key(params, payload) else {
        panic!("the reference derivation succeeds")
    };

    // Assert
    assert_eq!(success.key.expose(), &expected);
}

/// Contract: the capsule-randomness purpose's context string, the
///   serialization, and the output match the independent implementation.
/// Arrange: the reference key material and the reference context.
/// Act:     `kdf.derive_key(params, payload)` for `CapsuleRandomness`,
///   length 64.
/// Assert:  `success.key.expose()` equals the capsule-randomness vector.
#[test]
fn derive_key_for_the_capsule_randomness_matches_the_independent_vector() {
    // Arrange
    let Ok(kdf) = Blake3KeyedKdf::try_new(Blake3KeyedKdfConstructorParams);
    let secret = build_secret(SecretConstructorParamsOverrides {
        value: Some(vec![0x42u8; 32]),
    });
    let Ok(context) = decode(REFERENCE_CONTEXT_HEX) else {
        panic!("the reference context is valid hex")
    };
    let params = build_derive_key_params(DeriveKeyParamsOverrides {
        purpose: Some(DerivationPurpose::CapsuleRandomness),
        length: Some(64),
    });
    let payload = DeriveKeyPayload {
        key_material: &secret,
        context: &context,
    };
    let Ok(expected) = decode(CAPSULE_RANDOMNESS_VECTOR_HEX) else {
        panic!("the vector is valid hex")
    };

    // Act
    let Ok(success) = kdf.derive_key(params, payload) else {
        panic!("the reference derivation succeeds")
    };

    // Assert
    assert_eq!(success.key.expose(), &expected);
}

/// Contract: the piece-group-key purpose's context string, the serialization,
///   and the output match the independent implementation.
/// Arrange: the reference key material and the reference context.
/// Act:     `kdf.derive_key(params, payload)` for `PieceGroupKey`, length 64.
/// Assert:  `success.key.expose()` equals the piece-group-key vector.
#[test]
fn derive_key_for_the_piece_group_key_matches_the_independent_vector() {
    // Arrange
    let Ok(kdf) = Blake3KeyedKdf::try_new(Blake3KeyedKdfConstructorParams);
    let secret = build_secret(SecretConstructorParamsOverrides {
        value: Some(vec![0x42u8; 32]),
    });
    let Ok(context) = decode(REFERENCE_CONTEXT_HEX) else {
        panic!("the reference context is valid hex")
    };
    let params = build_derive_key_params(DeriveKeyParamsOverrides {
        purpose: Some(DerivationPurpose::PieceGroupKey),
        length: Some(64),
    });
    let payload = DeriveKeyPayload {
        key_material: &secret,
        context: &context,
    };
    let Ok(expected) = decode(PIECE_GROUP_KEY_VECTOR_HEX) else {
        panic!("the vector is valid hex")
    };

    // Act
    let Ok(success) = kdf.derive_key(params, payload) else {
        panic!("the reference derivation succeeds")
    };

    // Assert
    assert_eq!(success.key.expose(), &expected);
}

/// Contract: the plaintext-root-key purpose's context string, the
///   serialization, and the output match the independent implementation.
/// Arrange: the reference key material and the reference context.
/// Act:     `kdf.derive_key(params, payload)` for `PlaintextRootKey`,
///   length 64.
/// Assert:  `success.key.expose()` equals the plaintext-root-key vector.
#[test]
fn derive_key_for_the_plaintext_root_key_matches_the_independent_vector() {
    // Arrange
    let Ok(kdf) = Blake3KeyedKdf::try_new(Blake3KeyedKdfConstructorParams);
    let secret = build_secret(SecretConstructorParamsOverrides {
        value: Some(vec![0x42u8; 32]),
    });
    let Ok(context) = decode(REFERENCE_CONTEXT_HEX) else {
        panic!("the reference context is valid hex")
    };
    let params = build_derive_key_params(DeriveKeyParamsOverrides {
        purpose: Some(DerivationPurpose::PlaintextRootKey),
        length: Some(64),
    });
    let payload = DeriveKeyPayload {
        key_material: &secret,
        context: &context,
    };
    let Ok(expected) = decode(PLAINTEXT_ROOT_KEY_VECTOR_HEX) else {
        panic!("the vector is valid hex")
    };

    // Act
    let Ok(success) = kdf.derive_key(params, payload) else {
        panic!("the reference derivation succeeds")
    };

    // Assert
    assert_eq!(success.key.expose(), &expected);
}

/// Contract: the requested length truncates the extendable output.
/// Arrange: the reference key material and the reference context.
/// Act:     `kdf.derive_key(params, payload)` for `WrappingKey`, length 32.
/// Assert:  `success.key.expose()` equals the first 32 bytes of the
///   wrapping-key vector.
#[test]
fn derive_key_of_thirty_two_bytes_is_the_prefix_of_the_sixty_four_byte_vector() {
    // Arrange
    let Ok(kdf) = Blake3KeyedKdf::try_new(Blake3KeyedKdfConstructorParams);
    let secret = build_secret(SecretConstructorParamsOverrides {
        value: Some(vec![0x42u8; 32]),
    });
    let Ok(context) = decode(REFERENCE_CONTEXT_HEX) else {
        panic!("the reference context is valid hex")
    };
    let params = build_derive_key_params(DeriveKeyParamsOverrides {
        purpose: Some(DerivationPurpose::WrappingKey),
        length: Some(32),
    });
    let payload = DeriveKeyPayload {
        key_material: &secret,
        context: &context,
    };
    let Ok(expected) = decode("549874404f0b25b65da55dbce0b410d0aa34085695af4b91303e2c1c0410f760")
    else {
        panic!("the prefix is valid hex")
    };

    // Act
    let Ok(success) = kdf.derive_key(params, payload) else {
        panic!("the reference derivation succeeds")
    };

    // Assert
    assert_eq!(success.key.expose(), &expected);
}

/// Contract: a requested length of zero yields an empty key.
/// Arrange: the reference key material and the reference context.
/// Act:     `kdf.derive_key(params, payload)` for `WrappingKey`, length 0.
/// Assert:  `success.key.expose()` is empty.
#[test]
fn derive_key_of_zero_bytes_returns_an_empty_key() {
    // Arrange
    let Ok(kdf) = Blake3KeyedKdf::try_new(Blake3KeyedKdfConstructorParams);
    let secret = build_secret(SecretConstructorParamsOverrides {
        value: Some(vec![0x42u8; 32]),
    });
    let Ok(context) = decode(REFERENCE_CONTEXT_HEX) else {
        panic!("the reference context is valid hex")
    };
    let params = build_derive_key_params(DeriveKeyParamsOverrides {
        purpose: Some(DerivationPurpose::WrappingKey),
        length: Some(0),
    });
    let payload = DeriveKeyPayload {
        key_material: &secret,
        context: &context,
    };

    // Act
    let Ok(success) = kdf.derive_key(params, payload) else {
        panic!("the zero-length derivation succeeds")
    };

    // Assert
    assert!(success.key.expose().is_empty());
}

/// Contract: moving a byte from the secret to the context changes the
///   derivation, because the secret's length is absorbed first.
/// Arrange: a 31-byte secret of `0x42` and the context `0x42` followed by the
///   reference context.
/// Act:     `kdf.derive_key(params, payload)` for `WrappingKey`, length 64.
/// Assert:  `success.key.expose()` differs from the wrapping-key vector.
#[test]
fn derive_key_binds_the_key_material_length() {
    // Arrange
    let Ok(kdf) = Blake3KeyedKdf::try_new(Blake3KeyedKdfConstructorParams);
    let secret = build_secret(SecretConstructorParamsOverrides {
        value: Some(vec![0x42u8; 31]),
    });
    let Ok(reference) = decode(REFERENCE_CONTEXT_HEX) else {
        panic!("the reference context is valid hex")
    };
    let mut context = vec![0x42u8];
    context.extend_from_slice(&reference);
    let params = build_derive_key_params(DeriveKeyParamsOverrides {
        purpose: Some(DerivationPurpose::WrappingKey),
        length: Some(64),
    });
    let payload = DeriveKeyPayload {
        key_material: &secret,
        context: &context,
    };
    let Ok(expected) = decode(WRAPPING_KEY_VECTOR_HEX) else {
        panic!("the vector is valid hex")
    };

    // Act
    let Ok(success) = kdf.derive_key(params, payload) else {
        panic!("the shifted-boundary derivation succeeds")
    };

    // Assert
    assert_ne!(success.key.expose(), &expected);
}

/// Contract: the concrete declares its KDF identifier, adapter version, and
///   interface version before any instance exists.
/// Arrange: none — `Blake3KeyedKdf::DECLARATION` is inherent.
/// Act:     read `Blake3KeyedKdf::DECLARATION`.
/// Assert:  `identifier` is `KdfIdentifier::Blake3KeyedV1`, `adapter_version`
///   is `1`, and `interface_version` is `KDF_INTERFACE_VERSION`.
#[test]
fn blake3_keyed_kdf_declares_its_identifier_and_versions() {
    // Arrange

    // Act
    let declaration = Blake3KeyedKdf::DECLARATION;

    // Assert
    assert!(matches!(
        declaration.identifier,
        KdfIdentifier::Blake3KeyedV1
    ));
    assert_eq!(declaration.adapter_version, 1);
    assert_eq!(declaration.interface_version, KDF_INTERFACE_VERSION);
}
