#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{Keccak256HashToScalar, Keccak256HashToScalarConstructorParams};
use crate::domain_tag::provides::{
    DomainTag, DomainTagConstructorParamsOverrides, build_domain_tag,
};
use crate::factory::provides::{
    HASH_TO_SCALAR_INTERFACE_VERSION, HashToScalarIdentifier, HashToScalarParams,
    HashToScalarPayload, IHashToScalarAdapter,
};
use hex::decode;
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingDeps, CreatePairingParamsOverrides,
    CreatePairingPayload, EncodeScalarParams, EncodeScalarPayload, IPairingAdapter,
    IPairingConsumer, PairingConcrete, build_create_pairing_params, create_pairing,
};

const REFERENCE_TAG: &[u8] = b"ChainTorrent hash-to-scalar test";

const REFERENCE_MESSAGE_HEX: &str = concat!(
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

const REFERENCE_BN254_SCALAR_HEX: &str =
    "10ed702d3a6c3c552391220194f0ed1c66d498f751ba2a51289840a3980f8de1";
const REFERENCE_BLS12_381_SCALAR_HEX: &str =
    "4151bea01b9ddc7edbe167b8167245798f08813fcb739ae26c7a3637880f8de2";
const EMPTY_MESSAGE_BN254_SCALAR_HEX: &str =
    "23d2a49fb7a5a80631ae7e64f5e30bc92f8fdf98ede1dfd472bf1333a046bb07";
const EMPTY_MESSAGE_BLS12_381_SCALAR_HEX: &str =
    "7176371812ceab64dfb5bd36f24695387ca1dcb7d4c9461a8246e9846046bb0a";

struct ScalarProbe {
    tag: DomainTag,
    message: Vec<u8>,
}

impl IPairingConsumer for ScalarProbe {
    type Output = Vec<u8>;

    fn consume_pairing<P: IPairingAdapter>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let Ok(hasher) = Keccak256HashToScalar::try_new(Keccak256HashToScalarConstructorParams);
        let Ok(success) = IHashToScalarAdapter::<P::Scalar>::hash_to_scalar(
            &hasher,
            HashToScalarParams { tag: &self.tag },
            HashToScalarPayload {
                message: &self.message,
            },
        ) else {
            panic!("the reference inputs hash")
        };
        let Ok(encoded) = payload.adapter.encode_scalar(
            EncodeScalarParams,
            EncodeScalarPayload {
                scalar: success.scalar,
            },
        );
        encoded.bytes.expose().as_ref().to_vec()
    }
}

/// Contract: the tag prefix, the tag, and the message hash and reduce modulo
///   the BN254 group order as the independent implementation does — the hashed
///   branch over arkworks.
/// Arrange: the reference tag, the 448-byte reference message, and
///   `PairingConcrete::Bn254Arkworks`.
/// Act:     `create_pairing` with the `ScalarProbe` consumer.
/// Assert:  `success.output` equals the reference message's independent BN254
///   scalar.
#[test]
fn hash_to_scalar_on_bn254_arkworks_reduces_the_reference_digest_to_the_independent_scalar() {
    // Arrange
    let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
        bytes: Some(REFERENCE_TAG.to_vec()),
    });
    let Ok(message) = decode(REFERENCE_MESSAGE_HEX) else {
        panic!("the reference message is valid hex")
    };
    let Ok(expected) = decode(REFERENCE_BN254_SCALAR_HEX) else {
        panic!("the independent scalar is valid hex")
    };
    let consumer = ScalarProbe { tag, message };
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Arkworks),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(
        &CreatePairingDeps { consumer },
        params,
        CreatePairingPayload,
    ) else {
        panic!("the pairing factory resolves the concrete")
    };

    // Assert
    assert_eq!(success.output, expected);
}

/// Contract: the tag prefix, the tag, and the message hash and reduce modulo
///   the BN254 group order as the independent implementation does — the hashed
///   branch over halo2curves.
/// Arrange: the reference tag, the 448-byte reference message, and
///   `PairingConcrete::Bn254Halo2curves`.
/// Act:     `create_pairing` with the `ScalarProbe` consumer.
/// Assert:  `success.output` equals the reference message's independent BN254
///   scalar.
#[test]
fn hash_to_scalar_on_bn254_halo2curves_reduces_the_reference_digest_to_the_independent_scalar() {
    // Arrange
    let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
        bytes: Some(REFERENCE_TAG.to_vec()),
    });
    let Ok(message) = decode(REFERENCE_MESSAGE_HEX) else {
        panic!("the reference message is valid hex")
    };
    let Ok(expected) = decode(REFERENCE_BN254_SCALAR_HEX) else {
        panic!("the independent scalar is valid hex")
    };
    let consumer = ScalarProbe { tag, message };
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Halo2curves),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(
        &CreatePairingDeps { consumer },
        params,
        CreatePairingPayload,
    ) else {
        panic!("the pairing factory resolves the concrete")
    };

    // Assert
    assert_eq!(success.output, expected);
}

/// Contract: the tag prefix, the tag, and the message hash and reduce modulo
///   the BLS12-381 group order as the independent implementation does — the
///   hashed branch over arkworks.
/// Arrange: the reference tag, the 448-byte reference message, and
///   `PairingConcrete::Bls12381Arkworks`.
/// Act:     `create_pairing` with the `ScalarProbe` consumer.
/// Assert:  `success.output` equals the reference message's independent
///   BLS12-381 scalar.
#[test]
fn hash_to_scalar_on_bls12_381_arkworks_reduces_the_reference_digest_to_the_independent_scalar() {
    // Arrange
    let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
        bytes: Some(REFERENCE_TAG.to_vec()),
    });
    let Ok(message) = decode(REFERENCE_MESSAGE_HEX) else {
        panic!("the reference message is valid hex")
    };
    let Ok(expected) = decode(REFERENCE_BLS12_381_SCALAR_HEX) else {
        panic!("the independent scalar is valid hex")
    };
    let consumer = ScalarProbe { tag, message };
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Arkworks),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(
        &CreatePairingDeps { consumer },
        params,
        CreatePairingPayload,
    ) else {
        panic!("the pairing factory resolves the concrete")
    };

    // Assert
    assert_eq!(success.output, expected);
}

/// Contract: the tag prefix, the tag, and the message hash and reduce modulo
///   the BLS12-381 group order as the independent implementation does — the
///   hashed branch over halo2curves.
/// Arrange: the reference tag, the 448-byte reference message, and
///   `PairingConcrete::Bls12381Halo2curves`.
/// Act:     `create_pairing` with the `ScalarProbe` consumer.
/// Assert:  `success.output` equals the reference message's independent
///   BLS12-381 scalar.
#[test]
fn hash_to_scalar_on_bls12_381_halo2curves_reduces_the_reference_digest_to_the_independent_scalar()
{
    // Arrange
    let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
        bytes: Some(REFERENCE_TAG.to_vec()),
    });
    let Ok(message) = decode(REFERENCE_MESSAGE_HEX) else {
        panic!("the reference message is valid hex")
    };
    let Ok(expected) = decode(REFERENCE_BLS12_381_SCALAR_HEX) else {
        panic!("the independent scalar is valid hex")
    };
    let consumer = ScalarProbe { tag, message };
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Halo2curves),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(
        &CreatePairingDeps { consumer },
        params,
        CreatePairingPayload,
    ) else {
        panic!("the pairing factory resolves the concrete")
    };

    // Assert
    assert_eq!(success.output, expected);
}

/// Contract: an empty message hashes and reduces — the hashed branch over a
///   digest at or above the BN254 group order.
/// Arrange: the reference tag, an empty message, and
///   `PairingConcrete::Bn254Arkworks`.
/// Act:     `create_pairing` with the `ScalarProbe` consumer.
/// Assert:  `success.output` equals the empty message's independent BN254
///   scalar.
#[test]
fn hash_to_scalar_on_bn254_arkworks_reduces_a_digest_above_the_group_order() {
    // Arrange
    let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
        bytes: Some(REFERENCE_TAG.to_vec()),
    });
    let Ok(expected) = decode(EMPTY_MESSAGE_BN254_SCALAR_HEX) else {
        panic!("the independent scalar is valid hex")
    };
    let consumer = ScalarProbe {
        tag,
        message: Vec::new(),
    };
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Arkworks),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(
        &CreatePairingDeps { consumer },
        params,
        CreatePairingPayload,
    ) else {
        panic!("the pairing factory resolves the concrete")
    };

    // Assert
    assert_eq!(success.output, expected);
}

/// Contract: an empty message hashes and reduces — the hashed branch over a
///   digest at or above the BLS12-381 group order.
/// Arrange: the reference tag, an empty message, and
///   `PairingConcrete::Bls12381Arkworks`.
/// Act:     `create_pairing` with the `ScalarProbe` consumer.
/// Assert:  `success.output` equals the empty message's independent BLS12-381
///   scalar.
#[test]
fn hash_to_scalar_on_bls12_381_arkworks_reduces_a_digest_above_the_group_order() {
    // Arrange
    let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
        bytes: Some(REFERENCE_TAG.to_vec()),
    });
    let Ok(expected) = decode(EMPTY_MESSAGE_BLS12_381_SCALAR_HEX) else {
        panic!("the independent scalar is valid hex")
    };
    let consumer = ScalarProbe {
        tag,
        message: Vec::new(),
    };
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Arkworks),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(
        &CreatePairingDeps { consumer },
        params,
        CreatePairingPayload,
    ) else {
        panic!("the pairing factory resolves the concrete")
    };

    // Assert
    assert_eq!(success.output, expected);
}

/// Contract: moving a byte from the tag to the message changes the scalar,
///   because the tag's length is absorbed first — the hashed branch
///   discriminates on the prefix.
/// Arrange: the tag `b"ChainTorrent hash-to-scalar tes"`, the message `0x74`
///   followed by the reference message, and `PairingConcrete::Bn254Arkworks`.
/// Act:     `create_pairing` with the `ScalarProbe` consumer.
/// Assert:  `success.output` differs from the reference message's BN254 scalar.
#[test]
fn hash_to_scalar_binds_the_tag_length() {
    // Arrange
    let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
        bytes: Some(b"ChainTorrent hash-to-scalar tes".to_vec()),
    });
    let Ok(reference) = decode(REFERENCE_MESSAGE_HEX) else {
        panic!("the reference message is valid hex")
    };
    let mut message = vec![0x74];
    message.extend_from_slice(&reference);
    let Ok(expected) = decode(REFERENCE_BN254_SCALAR_HEX) else {
        panic!("the independent scalar is valid hex")
    };
    let consumer = ScalarProbe { tag, message };
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Arkworks),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(
        &CreatePairingDeps { consumer },
        params,
        CreatePairingPayload,
    ) else {
        panic!("the pairing factory resolves the concrete")
    };

    // Assert
    assert_ne!(success.output, expected);
}

/// Contract: a different tag over the same message yields a different scalar —
///   the hashed branch separates domains.
/// Arrange: the tag `b"ChainTorrent other tag"`, the 448-byte reference
///   message, and `PairingConcrete::Bn254Arkworks`.
/// Act:     `create_pairing` with the `ScalarProbe` consumer.
/// Assert:  `success.output` differs from the reference message's BN254 scalar.
#[test]
fn hash_to_scalar_separates_domains() {
    // Arrange
    let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
        bytes: Some(b"ChainTorrent other tag".to_vec()),
    });
    let Ok(message) = decode(REFERENCE_MESSAGE_HEX) else {
        panic!("the reference message is valid hex")
    };
    let Ok(expected) = decode(REFERENCE_BN254_SCALAR_HEX) else {
        panic!("the independent scalar is valid hex")
    };
    let consumer = ScalarProbe { tag, message };
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Arkworks),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(
        &CreatePairingDeps { consumer },
        params,
        CreatePairingPayload,
    ) else {
        panic!("the pairing factory resolves the concrete")
    };

    // Assert
    assert_ne!(success.output, expected);
}

/// Contract: the concrete declares its identifier and versions — the
///   `DECLARATION` constant reads the family's values before any instance
///   exists.
/// Arrange: none; the declaration is inherent.
/// Act:     read `Keccak256HashToScalar::DECLARATION`.
/// Assert:  `identifier` matches `HashToScalarIdentifier::Keccak256V1`,
///   `adapter_version` equals `1`, and `interface_version` equals
///   `HASH_TO_SCALAR_INTERFACE_VERSION`.
#[test]
fn keccak256_hash_to_scalar_declares_its_identifier_and_versions() {
    // Arrange

    // Act
    let declaration = Keccak256HashToScalar::DECLARATION;

    // Assert
    assert!(matches!(
        declaration.identifier,
        HashToScalarIdentifier::Keccak256V1
    ));
    assert_eq!(declaration.adapter_version, 1);
    assert_eq!(
        declaration.interface_version,
        HASH_TO_SCALAR_INTERFACE_VERSION
    );
}
