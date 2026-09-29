#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    Bls12381ArkworksPairing, Bls12381ArkworksPairingConstructorParams, Bls12381ArkworksScalar,
};
use crate::factory::provides::{
    AddG1Params, AddG1PayloadOverrides, AddG2Params, AddG2PayloadOverrides, DecodeG1ErrorReturn,
    DecodeG1Params, DecodeG2ErrorReturn, DecodeG2Params, DecodeScalarErrorReturn,
    DecodeScalarParams, EncodeG1Params, EncodeG1PayloadOverrides, EncodeG2Params,
    EncodeG2PayloadOverrides, EncodeScalarParams, EncodeScalarPayloadOverrides, G1GeneratorParams,
    G1GeneratorPayload, G2GeneratorParams, G2GeneratorPayload, IPairingAdapter,
    ISampleUniformScalar, MsmG1Params, MsmG1PayloadOverrides, MsmG1TermOverrides, MsmG2Params,
    MsmG2PayloadOverrides, MsmG2TermOverrides, MulG1Params, MulG1PayloadOverrides, MulG2Params,
    MulG2PayloadOverrides, PAIRING_INTERFACE_VERSION, PairingCurve, PairingProductIsOneParams,
    PairingProductIsOnePayloadOverrides, PairingProductTermOverrides, PrecompileEncoding,
    SampleUniformScalarErrorReturn, SampleUniformScalarParams, SampleUniformScalarPayloadOverrides,
    VerifierGroupArithmetic, build_add_g1_payload, build_add_g2_payload, build_encode_g1_payload,
    build_encode_g2_payload, build_encode_scalar_payload, build_msm_g1_payload, build_msm_g1_term,
    build_msm_g2_payload, build_msm_g2_term, build_mul_g1_payload, build_mul_g2_payload,
    build_pairing_product_is_one_payload, build_pairing_product_term,
    build_sample_uniform_scalar_payload,
};
use ark_bls12_381::{Fq, Fq2, G1Affine, G2Affine};
use ark_ec::AffineRepr;
use ark_ff::{BigInteger, PrimeField};
use domain::{SecretConstructorParamsOverrides, build_secret};
use hex::decode;
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParamsOverrides, CreateRandomSourcePayload,
    FillBytesParams, FillBytesPayloadOverrides, RandomSourceKind,
    build_create_random_source_params, build_fill_bytes_payload, create_random_source,
};

const BASE_FIELD_MODULUS_PADDED_HEX: &str = "00000000000000000000000000000000\
     1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab";
const GROUP_ORDER_HEX: &str = "73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001";
const GROUP_ORDER_MINUS_ONE_HEX: &str =
    "73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000000";
const GROUP_ORDER_MINUS_TWO_HEX: &str =
    "73eda753299d7d483339d80809a1d80553bda402fffe5bfefffffffeffffffff";
const G1_GENERATOR_HEX: &str = "00000000000000000000000000000000\
     17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb\
     00000000000000000000000000000000\
     08b3f481e3aaa0f1a09e30ed741d8ae4fcf5e095d5d00af600db18cb2c04b3edd03cc744a2888ae40caa232946c5e7e1";
const G1_GENERATOR_OFF_CURVE_HEX: &str = "00000000000000000000000000000000\
     17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb\
     00000000000000000000000000000000\
     08b3f481e3aaa0f1a09e30ed741d8ae4fcf5e095d5d00af600db18cb2c04b3edd03cc744a2888ae40caa232946c5e7e2";
const G2_GENERATOR_HEX: &str = "00000000000000000000000000000000\
     024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8\
     00000000000000000000000000000000\
     13e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e\
     00000000000000000000000000000000\
     0ce5d527727d6e118cc9cdc6da2e351aadfd9baa8cbdd3a76d429a695160d12c923ac9cc3baca289e193548608b82801\
     00000000000000000000000000000000\
     0606c4a02ea734cc32acd2b02bc28b99cb3e287e85a763af267492ab572e99ab3f370d275cec1da1aaa9075ff05f79be";
const G2_GENERATOR_OFF_CURVE_HEX: &str = "00000000000000000000000000000000\
     024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8\
     00000000000000000000000000000000\
     13e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e\
     00000000000000000000000000000000\
     0ce5d527727d6e118cc9cdc6da2e351aadfd9baa8cbdd3a76d429a695160d12c923ac9cc3baca289e193548608b82801\
     00000000000000000000000000000000\
     0606c4a02ea734cc32acd2b02bc28b99cb3e287e85a763af267492ab572e99ab3f370d275cec1da1aaa9075ff05f79bf";
const SCALAR_TWO_HEX: &str = "0000000000000000000000000000000000000000000000000000000000000002";
const SCALAR_THREE_HEX: &str = "0000000000000000000000000000000000000000000000000000000000000003";
const SCALAR_FIVE_HEX: &str = "0000000000000000000000000000000000000000000000000000000000000005";

/// Contract: the generated branch returns the first-group generator, whose
///   precompile encoding is EIP-2537's 128 bytes — the 64-byte x then the
///   64-byte y of the generator, each sixteen zero bytes followed by the
///   48-byte big-endian integer.
/// Arrange: the pairing adapter.
/// Act:     `pairing.g1_generator`, then `pairing.encode_g1` over it.
/// Assert:  the encoded bytes equal the EIP-2537 generator vector.
#[test]
fn g1_generator_encodes_to_the_eip_2537_generator() {
    // Arrange
    let Ok(pairing) = Bls12381ArkworksPairing::try_new(Bls12381ArkworksPairingConstructorParams);
    let Ok(expected) = decode(G1_GENERATOR_HEX) else {
        panic!("the vector decodes")
    };

    // Act
    let Ok(generated) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
    let Ok(encoded) = pairing.encode_g1(
        EncodeG1Params,
        build_encode_g1_payload(EncodeG1PayloadOverrides {
            point: Some(generated.point),
        }),
    );

    // Assert
    assert_eq!(encoded.bytes, expected);
}

/// Contract: the generated branch returns the second-group generator, whose
///   precompile encoding is EIP-2537's 256 bytes — each coordinate's `c0`
///   before its `c1`, each padded to 64 bytes.
/// Arrange: the pairing adapter.
/// Act:     `pairing.g2_generator`, then `pairing.encode_g2` over it.
/// Assert:  the encoded bytes equal the EIP-2537 generator vector.
#[test]
fn g2_generator_encodes_to_the_eip_2537_generator() {
    // Arrange
    let Ok(pairing) = Bls12381ArkworksPairing::try_new(Bls12381ArkworksPairingConstructorParams);
    let Ok(expected) = decode(G2_GENERATOR_HEX) else {
        panic!("the vector decodes")
    };

    // Act
    let Ok(generated) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
    let Ok(encoded) = pairing.encode_g2(
        EncodeG2Params,
        build_encode_g2_payload(EncodeG2PayloadOverrides {
            point: Some(generated.point),
        }),
    );

    // Assert
    assert_eq!(encoded.bytes, expected);
}

/// Contract: the valid branch of `decode_g1` holds the point, which
///   re-encodes to the same EIP-2537 bytes.
/// Arrange: the pairing adapter and the EIP-2537 first-group generator
///   vector.
/// Act:     `pairing.decode_g1` over the vector, then `pairing.encode_g1`.
/// Assert:  the encoded bytes equal the vector.
#[test]
fn decode_g1_round_trips_the_eip_2537_generator() {
    // Arrange
    let Ok(pairing) = Bls12381ArkworksPairing::try_new(Bls12381ArkworksPairingConstructorParams);
    let Ok(bytes) = decode(G1_GENERATOR_HEX) else {
        panic!("the vector decodes")
    };

    // Act
    let Ok(decoded) = pairing.decode_g1(DecodeG1Params, &bytes) else {
        panic!("the vector decodes to a point")
    };
    let Ok(encoded) = pairing.encode_g1(
        EncodeG1Params,
        build_encode_g1_payload(EncodeG1PayloadOverrides {
            point: Some(decoded.point),
        }),
    );

    // Assert
    assert_eq!(encoded.bytes, bytes);
}

/// Contract: the valid branch of `decode_g2` holds the point, which
///   re-encodes to the same EIP-2537 bytes.
/// Arrange: the pairing adapter and the EIP-2537 second-group generator
///   vector.
/// Act:     `pairing.decode_g2` over the vector, then `pairing.encode_g2`.
/// Assert:  the encoded bytes equal the vector.
#[test]
fn decode_g2_round_trips_the_eip_2537_generator() {
    // Arrange
    let Ok(pairing) = Bls12381ArkworksPairing::try_new(Bls12381ArkworksPairingConstructorParams);
    let Ok(bytes) = decode(G2_GENERATOR_HEX) else {
        panic!("the vector decodes")
    };

    // Act
    let Ok(decoded) = pairing.decode_g2(DecodeG2Params, &bytes) else {
        panic!("the vector decodes to a point")
    };
    let Ok(encoded) = pairing.encode_g2(
        EncodeG2Params,
        build_encode_g2_payload(EncodeG2PayloadOverrides {
            point: Some(decoded.point),
        }),
    );

    // Assert
    assert_eq!(encoded.bytes, bytes);
}

/// Contract: the identity branch of `decode_g1` — both coordinates zero —
///   holds the identity, which adds nothing to the generator.
/// Arrange: the pairing adapter, 128 zero bytes, and the EIP-2537
///   first-group generator vector.
/// Act:     `pairing.decode_g1` over the zeros, `pairing.add_g1` of the
///   generator and the decoded point, then `pairing.encode_g1`.
/// Assert:  the encoded bytes equal the EIP-2537 generator vector.
#[test]
fn decode_g1_reads_the_all_zero_encoding_as_the_identity() {
    // Arrange
    let Ok(pairing) = Bls12381ArkworksPairing::try_new(Bls12381ArkworksPairingConstructorParams);
    let Ok(expected) = decode(G1_GENERATOR_HEX) else {
        panic!("the vector decodes")
    };
    let zeros = vec![0u8; 128];
    let Ok(generated) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);

    // Act
    let Ok(decoded) = pairing.decode_g1(DecodeG1Params, &zeros) else {
        panic!("the all-zero encoding decodes to the identity")
    };
    let Ok(added) = pairing.add_g1(
        AddG1Params,
        build_add_g1_payload(AddG1PayloadOverrides {
            left: Some(generated.point),
            right: Some(decoded.point),
        }),
    );
    let Ok(encoded) = pairing.encode_g1(
        EncodeG1Params,
        build_encode_g1_payload(EncodeG1PayloadOverrides {
            point: Some(added.sum),
        }),
    );

    // Assert
    assert_eq!(encoded.bytes, expected);
}

/// Contract: the identity branch of `decode_g2` — all four coordinates
///   zero — holds the identity, which adds nothing to the generator.
/// Arrange: the pairing adapter, 256 zero bytes, and the EIP-2537
///   second-group generator vector.
/// Act:     `pairing.decode_g2` over the zeros, `pairing.add_g2` of the
///   generator and the decoded point, then `pairing.encode_g2`.
/// Assert:  the encoded bytes equal the EIP-2537 generator vector.
#[test]
fn decode_g2_reads_the_all_zero_encoding_as_the_identity() {
    // Arrange
    let Ok(pairing) = Bls12381ArkworksPairing::try_new(Bls12381ArkworksPairingConstructorParams);
    let Ok(expected) = decode(G2_GENERATOR_HEX) else {
        panic!("the vector decodes")
    };
    let zeros = vec![0u8; 256];
    let Ok(generated) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);

    // Act
    let Ok(decoded) = pairing.decode_g2(DecodeG2Params, &zeros) else {
        panic!("the all-zero encoding decodes to the identity")
    };
    let Ok(added) = pairing.add_g2(
        AddG2Params,
        build_add_g2_payload(AddG2PayloadOverrides {
            left: Some(generated.point),
            right: Some(decoded.point),
        }),
    );
    let Ok(encoded) = pairing.encode_g2(
        EncodeG2Params,
        build_encode_g2_payload(EncodeG2PayloadOverrides {
            point: Some(added.sum),
        }),
    );

    // Assert
    assert_eq!(encoded.bytes, expected);
}

/// Contract: the wrong-length branch of `decode_g1` returns
///   `WrongLength { expected: 128, actual }`.
/// Arrange: the pairing adapter and 127 bytes.
/// Act:     `pairing.decode_g1` over the bytes.
/// Assert:  the return is `Err(DecodeG1ErrorReturn::WrongLength { expected:
///   128, actual: 127 })`.
#[test]
fn decode_g1_rejects_a_wrong_length() {
    // Arrange
    let Ok(pairing) = Bls12381ArkworksPairing::try_new(Bls12381ArkworksPairingConstructorParams);
    let bytes = vec![0u8; 127];

    // Act
    let result = pairing.decode_g1(DecodeG1Params, &bytes);

    // Assert
    assert!(matches!(
        result,
        Err(DecodeG1ErrorReturn::WrongLength {
            expected: 128,
            actual: 127
        })
    ));
}

/// Contract: the non-canonical-coordinate branch of `decode_g1` — a nonzero
///   byte among a coordinate's first 16 padding bytes — returns
///   `NonCanonicalCoordinate`.
/// Arrange: the pairing adapter and the first-group generator vector with
///   its first byte set to `01`.
/// Act:     `pairing.decode_g1` over the bytes.
/// Assert:  the return is `Err(DecodeG1ErrorReturn::NonCanonicalCoordinate)`.
#[test]
fn decode_g1_rejects_a_nonzero_padding_byte() {
    // Arrange
    let Ok(pairing) = Bls12381ArkworksPairing::try_new(Bls12381ArkworksPairingConstructorParams);
    let Ok(mut bytes) = decode(G1_GENERATOR_HEX) else {
        panic!("the vector decodes")
    };
    bytes[0] = 0x01;

    // Act
    let result = pairing.decode_g1(DecodeG1Params, &bytes);

    // Assert
    assert!(matches!(
        result,
        Err(DecodeG1ErrorReturn::NonCanonicalCoordinate)
    ));
}

/// Contract: the non-canonical-coordinate branch of `decode_g1` — a
///   coordinate's last 48 bytes at least the base field modulus — returns
///   `NonCanonicalCoordinate`.
/// Arrange: the pairing adapter, the padded base field modulus, and the
///   first-group generator's padded `y`.
/// Act:     `pairing.decode_g1` over the concatenated bytes.
/// Assert:  the return is `Err(DecodeG1ErrorReturn::NonCanonicalCoordinate)`.
#[test]
fn decode_g1_rejects_a_coordinate_equal_to_the_base_field_modulus() {
    // Arrange
    let Ok(pairing) = Bls12381ArkworksPairing::try_new(Bls12381ArkworksPairingConstructorParams);
    let Ok(mut bytes) = decode(BASE_FIELD_MODULUS_PADDED_HEX) else {
        panic!("the modulus decodes")
    };
    let Ok(generator) = decode(G1_GENERATOR_HEX) else {
        panic!("the vector decodes")
    };
    bytes.extend_from_slice(&generator[64..128]);

    // Act
    let result = pairing.decode_g1(DecodeG1Params, &bytes);

    // Assert
    assert!(matches!(
        result,
        Err(DecodeG1ErrorReturn::NonCanonicalCoordinate)
    ));
}

/// Contract: the off-the-curve branch of `decode_g1` — a canonical pair not
///   on the curve — returns `NotOnCurve`.
/// Arrange: the pairing adapter and the first-group generator vector with
///   its last byte `e1` replaced by `e2`.
/// Act:     `pairing.decode_g1` over the bytes.
/// Assert:  the return is `Err(DecodeG1ErrorReturn::NotOnCurve)`.
#[test]
fn decode_g1_rejects_a_point_off_the_curve() {
    // Arrange
    let Ok(pairing) = Bls12381ArkworksPairing::try_new(Bls12381ArkworksPairingConstructorParams);
    let Ok(bytes) = decode(G1_GENERATOR_OFF_CURVE_HEX) else {
        panic!("the vector decodes")
    };

    // Act
    let result = pairing.decode_g1(DecodeG1Params, &bytes);

    // Assert
    assert!(matches!(result, Err(DecodeG1ErrorReturn::NotOnCurve)));
}

/// Contract: the outside-the-subgroup branch of `decode_g1` — an on-curve
///   point outside the prime-order subgroup — returns `NotInSubgroup`;
///   reachable, since BLS12-381's first group has a nontrivial cofactor.
/// Arrange: the pairing adapter and the first on-curve point outside the
///   subgroup found by `get_point_from_x_unchecked` over ascending `x`,
///   encoded as `x` then `y`, each 16 zero bytes followed by its 48
///   big-endian bytes.
/// Act:     `pairing.decode_g1` over the encoded point.
/// Assert:  the return is `Err(DecodeG1ErrorReturn::NotInSubgroup)`.
#[test]
fn decode_g1_rejects_a_point_outside_the_subgroup() {
    // Arrange
    let Ok(pairing) = Bls12381ArkworksPairing::try_new(Bls12381ArkworksPairingConstructorParams);
    let Some(point) = (1u64..).find_map(|c| {
        G1Affine::get_point_from_x_unchecked(Fq::from(c), false)
            .filter(|point| !point.is_in_correct_subgroup_assuming_on_curve())
    }) else {
        panic!("an on-curve point outside the subgroup exists")
    };
    let Some((x, y)) = point.xy() else {
        panic!("the point has coordinates")
    };
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&[0u8; 16]);
    bytes.extend_from_slice(&x.into_bigint().to_bytes_be());
    bytes.extend_from_slice(&[0u8; 16]);
    bytes.extend_from_slice(&y.into_bigint().to_bytes_be());

    // Act
    let result = pairing.decode_g1(DecodeG1Params, &bytes);

    // Assert
    assert!(matches!(result, Err(DecodeG1ErrorReturn::NotInSubgroup)));
}

/// Contract: the wrong-length branch of `decode_g2` returns
///   `WrongLength { expected: 256, actual }`.
/// Arrange: the pairing adapter and 255 bytes.
/// Act:     `pairing.decode_g2` over the bytes.
/// Assert:  the return is `Err(DecodeG2ErrorReturn::WrongLength { expected:
///   256, actual: 255 })`.
#[test]
fn decode_g2_rejects_a_wrong_length() {
    // Arrange
    let Ok(pairing) = Bls12381ArkworksPairing::try_new(Bls12381ArkworksPairingConstructorParams);
    let bytes = vec![0u8; 255];

    // Act
    let result = pairing.decode_g2(DecodeG2Params, &bytes);

    // Assert
    assert!(matches!(
        result,
        Err(DecodeG2ErrorReturn::WrongLength {
            expected: 256,
            actual: 255
        })
    ));
}

/// Contract: the off-the-curve branch of `decode_g2` returns `NotOnCurve`.
/// Arrange: the pairing adapter and the second-group generator vector with
///   its last byte `be` replaced by `bf`, a canonical pair off the curve.
/// Act:     `pairing.decode_g2` over the bytes.
/// Assert:  the return is `Err(DecodeG2ErrorReturn::NotOnCurve)`.
#[test]
fn decode_g2_rejects_a_point_off_the_curve() {
    // Arrange
    let Ok(pairing) = Bls12381ArkworksPairing::try_new(Bls12381ArkworksPairingConstructorParams);
    let Ok(bytes) = decode(G2_GENERATOR_OFF_CURVE_HEX) else {
        panic!("the vector decodes")
    };

    // Act
    let result = pairing.decode_g2(DecodeG2Params, &bytes);

    // Assert
    assert!(matches!(result, Err(DecodeG2ErrorReturn::NotOnCurve)));
}

/// Contract: the outside-the-subgroup branch of `decode_g2` — an on-curve
///   point outside the prime-order subgroup — returns `NotInSubgroup`.
/// Arrange: the pairing adapter and the first on-curve point outside the
///   subgroup found by `get_point_from_x_unchecked` over ascending `x.c0`,
///   encoded as `x.c0`, `x.c1`, `y.c0`, `y.c1`, each 16 zero bytes followed
///   by its 48 big-endian bytes.
/// Act:     `pairing.decode_g2` over the encoded point.
/// Assert:  the return is `Err(DecodeG2ErrorReturn::NotInSubgroup)`.
#[test]
fn decode_g2_rejects_a_point_outside_the_subgroup() {
    // Arrange
    let Ok(pairing) = Bls12381ArkworksPairing::try_new(Bls12381ArkworksPairingConstructorParams);
    let Some(point) = (1u64..).find_map(|c0| {
        G2Affine::get_point_from_x_unchecked(Fq2::new(Fq::from(c0), Fq::from(0u64)), false)
            .filter(|point| !point.is_in_correct_subgroup_assuming_on_curve())
    }) else {
        panic!("an on-curve point outside the subgroup exists")
    };
    let Some((x, y)) = point.xy() else {
        panic!("the point has coordinates")
    };
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&[0u8; 16]);
    bytes.extend_from_slice(&x.c0.into_bigint().to_bytes_be());
    bytes.extend_from_slice(&[0u8; 16]);
    bytes.extend_from_slice(&x.c1.into_bigint().to_bytes_be());
    bytes.extend_from_slice(&[0u8; 16]);
    bytes.extend_from_slice(&y.c0.into_bigint().to_bytes_be());
    bytes.extend_from_slice(&[0u8; 16]);
    bytes.extend_from_slice(&y.c1.into_bigint().to_bytes_be());

    // Act
    let result = pairing.decode_g2(DecodeG2Params, &bytes);

    // Assert
    assert!(matches!(result, Err(DecodeG2ErrorReturn::NotInSubgroup)));
}

/// Contract: the non-canonical branch of `decode_scalar` — bytes at least
///   the group order — returns `NonCanonical`.
/// Arrange: the pairing adapter and the group order's 32 bytes.
/// Act:     `pairing.decode_scalar` over the bytes.
/// Assert:  the return is `Err(DecodeScalarErrorReturn::NonCanonical)`.
#[test]
fn decode_scalar_rejects_the_group_order() {
    // Arrange
    let Ok(pairing) = Bls12381ArkworksPairing::try_new(Bls12381ArkworksPairingConstructorParams);
    let Ok(bytes) = decode(GROUP_ORDER_HEX) else {
        panic!("the vector decodes")
    };

    // Act
    let result = pairing.decode_scalar(DecodeScalarParams, &bytes);

    // Assert
    assert!(matches!(result, Err(DecodeScalarErrorReturn::NonCanonical)));
}

/// Contract: the valid branch of `decode_scalar` holds the scalar, which
///   re-encodes to the same 32 big-endian bytes.
/// Arrange: the pairing adapter and the group order minus one — the largest
///   canonical scalar.
/// Act:     `pairing.decode_scalar` over the bytes, then
///   `pairing.encode_scalar` over the result.
/// Assert:  the exposed encoded bytes equal the input.
#[test]
fn decode_scalar_round_trips_the_largest_canonical_scalar() {
    // Arrange
    let Ok(pairing) = Bls12381ArkworksPairing::try_new(Bls12381ArkworksPairingConstructorParams);
    let Ok(bytes) = decode(GROUP_ORDER_MINUS_ONE_HEX) else {
        panic!("the vector decodes")
    };

    // Act
    let Ok(decoded) = pairing.decode_scalar(DecodeScalarParams, &bytes) else {
        panic!("the largest canonical scalar decodes")
    };
    let Ok(encoded) = pairing.encode_scalar(
        EncodeScalarParams,
        build_encode_scalar_payload(EncodeScalarPayloadOverrides {
            scalar: Some(decoded.scalar),
        }),
    );

    // Assert
    assert_eq!(encoded.bytes.expose(), &bytes);
}

/// Contract: the wrong-length branch of `decode_scalar` returns
///   `WrongLength { expected: 32, actual }`.
/// Arrange: the pairing adapter and 31 bytes.
/// Act:     `pairing.decode_scalar` over the bytes.
/// Assert:  the return is `Err(DecodeScalarErrorReturn::WrongLength {
///   expected: 32, actual: 31 })`.
#[test]
fn decode_scalar_rejects_a_wrong_length() {
    // Arrange
    let Ok(pairing) = Bls12381ArkworksPairing::try_new(Bls12381ArkworksPairingConstructorParams);
    let bytes = vec![0u8; 31];

    // Act
    let result = pairing.decode_scalar(DecodeScalarParams, &bytes);

    // Assert
    assert!(matches!(
        result,
        Err(DecodeScalarErrorReturn::WrongLength {
            expected: 32,
            actual: 31
        })
    ));
}

/// Contract: the summed branch of `add_g1` returns the point's group sum —
///   the generator added to itself equals its multiple by two.
/// Arrange: the pairing adapter, the first-group generator, and the decoded
///   scalar two.
/// Act:     `pairing.add_g1` of the generator and itself, `pairing.mul_g1`
///   of the generator and two, each encoded.
/// Assert:  the two encodings are equal and differ from the generator's.
#[test]
fn add_g1_of_the_generator_to_itself_equals_its_multiple_by_two() {
    // Arrange
    let Ok(pairing) = Bls12381ArkworksPairing::try_new(Bls12381ArkworksPairingConstructorParams);
    let Ok(generator_bytes) = decode(G1_GENERATOR_HEX) else {
        panic!("the vector decodes")
    };
    let Ok(two_bytes) = decode(SCALAR_TWO_HEX) else {
        panic!("the vector decodes")
    };
    let Ok(generated) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
    let generator = generated.point;
    let Ok(two) = pairing.decode_scalar(DecodeScalarParams, &two_bytes) else {
        panic!("the scalar decodes")
    };

    // Act
    let Ok(added) = pairing.add_g1(
        AddG1Params,
        build_add_g1_payload(AddG1PayloadOverrides {
            left: Some(generator.clone()),
            right: Some(generator.clone()),
        }),
    );
    let Ok(multiplied) = pairing.mul_g1(
        MulG1Params,
        build_mul_g1_payload(MulG1PayloadOverrides {
            point: Some(generator),
            scalar: Some(two.scalar),
        }),
    );
    let Ok(sum_encoded) = pairing.encode_g1(
        EncodeG1Params,
        build_encode_g1_payload(EncodeG1PayloadOverrides {
            point: Some(added.sum),
        }),
    );
    let Ok(product_encoded) = pairing.encode_g1(
        EncodeG1Params,
        build_encode_g1_payload(EncodeG1PayloadOverrides {
            point: Some(multiplied.product),
        }),
    );

    // Assert
    assert_eq!(sum_encoded.bytes, product_encoded.bytes);
    assert_ne!(sum_encoded.bytes, generator_bytes);
}

/// Contract: the summed branch of `add_g2` returns the point's group sum —
///   the generator added to itself equals its multiple by two.
/// Arrange: the pairing adapter, the second-group generator, and the
///   decoded scalar two.
/// Act:     `pairing.add_g2` of the generator and itself, `pairing.mul_g2`
///   of the generator and two, each encoded.
/// Assert:  the two encodings are equal and differ from the generator's.
#[test]
fn add_g2_of_the_generator_to_itself_equals_its_multiple_by_two() {
    // Arrange
    let Ok(pairing) = Bls12381ArkworksPairing::try_new(Bls12381ArkworksPairingConstructorParams);
    let Ok(generator_bytes) = decode(G2_GENERATOR_HEX) else {
        panic!("the vector decodes")
    };
    let Ok(two_bytes) = decode(SCALAR_TWO_HEX) else {
        panic!("the vector decodes")
    };
    let Ok(generated) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
    let generator = generated.point;
    let Ok(two) = pairing.decode_scalar(DecodeScalarParams, &two_bytes) else {
        panic!("the scalar decodes")
    };

    // Act
    let Ok(added) = pairing.add_g2(
        AddG2Params,
        build_add_g2_payload(AddG2PayloadOverrides {
            left: Some(generator.clone()),
            right: Some(generator.clone()),
        }),
    );
    let Ok(multiplied) = pairing.mul_g2(
        MulG2Params,
        build_mul_g2_payload(MulG2PayloadOverrides {
            point: Some(generator),
            scalar: Some(two.scalar),
        }),
    );
    let Ok(sum_encoded) = pairing.encode_g2(
        EncodeG2Params,
        build_encode_g2_payload(EncodeG2PayloadOverrides {
            point: Some(added.sum),
        }),
    );
    let Ok(product_encoded) = pairing.encode_g2(
        EncodeG2Params,
        build_encode_g2_payload(EncodeG2PayloadOverrides {
            point: Some(multiplied.product),
        }),
    );

    // Assert
    assert_eq!(sum_encoded.bytes, product_encoded.bytes);
    assert_ne!(sum_encoded.bytes, generator_bytes);
}

/// Contract: the summed branch of `msm_g1` returns the terms' linear
///   combination — the generator weighted by two and by three equals its
///   multiple by five.
/// Arrange: the pairing adapter, the first-group generator, and the decoded
///   scalars two, three, and five.
/// Act:     `pairing.msm_g1` over terms (g1, 2) and (g1, 3), and
///   `pairing.mul_g1` of the generator and five, each encoded.
/// Assert:  the encodings are equal.
#[test]
fn msm_g1_equals_the_multiple_by_the_sum_of_its_scalars() {
    // Arrange
    let Ok(pairing) = Bls12381ArkworksPairing::try_new(Bls12381ArkworksPairingConstructorParams);
    let Ok(two_bytes) = decode(SCALAR_TWO_HEX) else {
        panic!("the vector decodes")
    };
    let Ok(three_bytes) = decode(SCALAR_THREE_HEX) else {
        panic!("the vector decodes")
    };
    let Ok(five_bytes) = decode(SCALAR_FIVE_HEX) else {
        panic!("the vector decodes")
    };
    let Ok(generated) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
    let generator = generated.point;
    let Ok(two) = pairing.decode_scalar(DecodeScalarParams, &two_bytes) else {
        panic!("the scalar decodes")
    };
    let Ok(three) = pairing.decode_scalar(DecodeScalarParams, &three_bytes) else {
        panic!("the scalar decodes")
    };
    let Ok(five) = pairing.decode_scalar(DecodeScalarParams, &five_bytes) else {
        panic!("the scalar decodes")
    };
    let terms = vec![
        build_msm_g1_term(MsmG1TermOverrides {
            base: Some(generator.clone()),
            scalar: Some(two.scalar),
        }),
        build_msm_g1_term(MsmG1TermOverrides {
            base: Some(generator.clone()),
            scalar: Some(three.scalar),
        }),
    ];

    // Act
    let Ok(summed) = pairing.msm_g1(
        MsmG1Params,
        build_msm_g1_payload(MsmG1PayloadOverrides { terms: Some(terms) }),
    );
    let Ok(multiplied) = pairing.mul_g1(
        MulG1Params,
        build_mul_g1_payload(MulG1PayloadOverrides {
            point: Some(generator),
            scalar: Some(five.scalar),
        }),
    );
    let Ok(sum_encoded) = pairing.encode_g1(
        EncodeG1Params,
        build_encode_g1_payload(EncodeG1PayloadOverrides {
            point: Some(summed.sum),
        }),
    );
    let Ok(product_encoded) = pairing.encode_g1(
        EncodeG1Params,
        build_encode_g1_payload(EncodeG1PayloadOverrides {
            point: Some(multiplied.product),
        }),
    );

    // Assert
    assert_eq!(sum_encoded.bytes, product_encoded.bytes);
}

/// Contract: the summed branch of `msm_g2` returns the terms' linear
///   combination — the generator weighted by two and by three equals its
///   multiple by five.
/// Arrange: the pairing adapter, the second-group generator, and the decoded
///   scalars two, three, and five.
/// Act:     `pairing.msm_g2` over terms (g2, 2) and (g2, 3), and
///   `pairing.mul_g2` of the generator and five, each encoded.
/// Assert:  the encodings are equal.
#[test]
fn msm_g2_equals_the_multiple_by_the_sum_of_its_scalars() {
    // Arrange
    let Ok(pairing) = Bls12381ArkworksPairing::try_new(Bls12381ArkworksPairingConstructorParams);
    let Ok(two_bytes) = decode(SCALAR_TWO_HEX) else {
        panic!("the vector decodes")
    };
    let Ok(three_bytes) = decode(SCALAR_THREE_HEX) else {
        panic!("the vector decodes")
    };
    let Ok(five_bytes) = decode(SCALAR_FIVE_HEX) else {
        panic!("the vector decodes")
    };
    let Ok(generated) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
    let generator = generated.point;
    let Ok(two) = pairing.decode_scalar(DecodeScalarParams, &two_bytes) else {
        panic!("the scalar decodes")
    };
    let Ok(three) = pairing.decode_scalar(DecodeScalarParams, &three_bytes) else {
        panic!("the scalar decodes")
    };
    let Ok(five) = pairing.decode_scalar(DecodeScalarParams, &five_bytes) else {
        panic!("the scalar decodes")
    };
    let terms = vec![
        build_msm_g2_term(MsmG2TermOverrides {
            base: Some(generator.clone()),
            scalar: Some(two.scalar),
        }),
        build_msm_g2_term(MsmG2TermOverrides {
            base: Some(generator.clone()),
            scalar: Some(three.scalar),
        }),
    ];

    // Act
    let Ok(summed) = pairing.msm_g2(
        MsmG2Params,
        build_msm_g2_payload(MsmG2PayloadOverrides { terms: Some(terms) }),
    );
    let Ok(multiplied) = pairing.mul_g2(
        MulG2Params,
        build_mul_g2_payload(MulG2PayloadOverrides {
            point: Some(generator),
            scalar: Some(five.scalar),
        }),
    );
    let Ok(sum_encoded) = pairing.encode_g2(
        EncodeG2Params,
        build_encode_g2_payload(EncodeG2PayloadOverrides {
            point: Some(summed.sum),
        }),
    );
    let Ok(product_encoded) = pairing.encode_g2(
        EncodeG2Params,
        build_encode_g2_payload(EncodeG2PayloadOverrides {
            point: Some(multiplied.product),
        }),
    );

    // Assert
    assert_eq!(sum_encoded.bytes, product_encoded.bytes);
}

/// Contract: the evaluated branch of `pairing_product_is_one` — a pairing
///   and its inverse in the product — returns `is_one: true`.
/// Arrange: the pairing adapter, both generators, and the decoded scalar
///   `r - 1`; the terms are `(g1, g2)` and `(g1 · (r - 1), g2)`.
/// Act:     `pairing.pairing_product_is_one` over the terms.
/// Assert:  `is_one` is `true`.
#[test]
fn pairing_product_is_one_for_a_pairing_and_its_inverse() {
    // Arrange
    let Ok(pairing) = Bls12381ArkworksPairing::try_new(Bls12381ArkworksPairingConstructorParams);
    let Ok(r_minus_one_bytes) = decode(GROUP_ORDER_MINUS_ONE_HEX) else {
        panic!("the vector decodes")
    };
    let Ok(g1) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
    let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
    let Ok(r_minus_one) = pairing.decode_scalar(DecodeScalarParams, &r_minus_one_bytes) else {
        panic!("the scalar decodes")
    };
    let Ok(inverse_g1) = pairing.mul_g1(
        MulG1Params,
        build_mul_g1_payload(MulG1PayloadOverrides {
            point: Some(g1.point.clone()),
            scalar: Some(r_minus_one.scalar),
        }),
    );
    let terms = vec![
        build_pairing_product_term(PairingProductTermOverrides {
            g1: Some(g1.point),
            g2: Some(g2.point.clone()),
        }),
        build_pairing_product_term(PairingProductTermOverrides {
            g1: Some(inverse_g1.product),
            g2: Some(g2.point),
        }),
    ];

    // Act
    let Ok(result) = pairing.pairing_product_is_one(
        PairingProductIsOneParams,
        build_pairing_product_is_one_payload(PairingProductIsOnePayloadOverrides {
            terms: Some(terms),
        }),
    );

    // Assert
    assert!(result.is_one);
}

/// Contract: the evaluated branch of `pairing_product_is_one` — a lone
///   generator pairing — returns `is_one: false`.
/// Arrange: the pairing adapter, both generators, and the one term
///   `(g1, g2)`.
/// Act:     `pairing.pairing_product_is_one` over the term.
/// Assert:  `is_one` is `false`.
#[test]
fn pairing_product_is_not_one_for_a_single_generator_pairing() {
    // Arrange
    let Ok(pairing) = Bls12381ArkworksPairing::try_new(Bls12381ArkworksPairingConstructorParams);
    let Ok(g1) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
    let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
    let terms = vec![build_pairing_product_term(PairingProductTermOverrides {
        g1: Some(g1.point),
        g2: Some(g2.point),
    })];

    // Act
    let Ok(result) = pairing.pairing_product_is_one(
        PairingProductIsOneParams,
        build_pairing_product_is_one_payload(PairingProductIsOnePayloadOverrides {
            terms: Some(terms),
        }),
    );

    // Assert
    assert!(!result.is_one);
}

/// Contract: the evaluated branch of `pairing_product_is_one` — the scalar
///   moved bilinearly between the two source groups — returns
///   `is_one: true` for `(g1 · 2, g2)` with `(g1, g2 · (r - 2))`.
/// Arrange: the pairing adapter, both generators, and the decoded scalars
///   two and `r - 2`.
/// Act:     `pairing.pairing_product_is_one` over the terms.
/// Assert:  `is_one` is `true`.
#[test]
fn pairing_product_is_one_across_the_bilinear_exchange() {
    // Arrange
    let Ok(pairing) = Bls12381ArkworksPairing::try_new(Bls12381ArkworksPairingConstructorParams);
    let Ok(two_bytes) = decode(SCALAR_TWO_HEX) else {
        panic!("the vector decodes")
    };
    let Ok(r_minus_two_bytes) = decode(GROUP_ORDER_MINUS_TWO_HEX) else {
        panic!("the vector decodes")
    };
    let Ok(g1) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
    let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
    let Ok(two) = pairing.decode_scalar(DecodeScalarParams, &two_bytes) else {
        panic!("the scalar decodes")
    };
    let Ok(r_minus_two) = pairing.decode_scalar(DecodeScalarParams, &r_minus_two_bytes) else {
        panic!("the scalar decodes")
    };
    let Ok(g1_times_two) = pairing.mul_g1(
        MulG1Params,
        build_mul_g1_payload(MulG1PayloadOverrides {
            point: Some(g1.point.clone()),
            scalar: Some(two.scalar),
        }),
    );
    let Ok(g2_times_r_minus_two) = pairing.mul_g2(
        MulG2Params,
        build_mul_g2_payload(MulG2PayloadOverrides {
            point: Some(g2.point.clone()),
            scalar: Some(r_minus_two.scalar),
        }),
    );
    let terms = vec![
        build_pairing_product_term(PairingProductTermOverrides {
            g1: Some(g1_times_two.product),
            g2: Some(g2.point.clone()),
        }),
        build_pairing_product_term(PairingProductTermOverrides {
            g1: Some(g1.point),
            g2: Some(g2_times_r_minus_two.product),
        }),
    ];

    // Act
    let Ok(result) = pairing.pairing_product_is_one(
        PairingProductIsOneParams,
        build_pairing_product_is_one_payload(PairingProductIsOnePayloadOverrides {
            terms: Some(terms),
        }),
    );

    // Assert
    assert!(result.is_one);
}

/// Contract: the wrong-length branch of `sample_from_uniform_bytes` returns
///   `WrongLength { expected: 64, actual }`.
/// Arrange: a uniform payload holding 63 bytes, differing from
///   `UNIFORM_BYTES_LENGTH`.
/// Act:     `Bls12381ArkworksScalar::sample_from_uniform_bytes` over the
///   payload.
/// Assert:  the return is `Err(SampleUniformScalarErrorReturn::WrongLength
///   { expected: 64, actual: 63 })`.
#[test]
fn sample_from_uniform_bytes_rejects_a_wrong_length() {
    // Arrange
    let payload = build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
        uniform: Some(build_secret(SecretConstructorParamsOverrides {
            value: Some(vec![0u8; 63]),
        })),
    });

    // Act
    let result =
        Bls12381ArkworksScalar::sample_from_uniform_bytes(SampleUniformScalarParams, payload);

    // Assert
    assert!(matches!(
        result,
        Err(SampleUniformScalarErrorReturn::WrongLength {
            expected: 64,
            actual: 63
        })
    ));
}

/// Contract: the sampled branch of `sample_from_uniform_bytes` reads the
///   input as one big-endian integer — 63 zero bytes followed by `05` yield
///   the scalar five.
/// Arrange: the pairing adapter and a uniform payload holding 63 zero bytes
///   followed by `05`.
/// Act:     `Bls12381ArkworksScalar::sample_from_uniform_bytes` over the
///   payload, then `pairing.encode_scalar` of a clone of the exposed
///   scalar.
/// Assert:  the exposed encoded bytes equal the 32-byte big-endian encoding
///   of five.
#[test]
fn sample_from_uniform_bytes_reads_its_input_as_a_big_endian_integer() {
    // Arrange
    let Ok(pairing) = Bls12381ArkworksPairing::try_new(Bls12381ArkworksPairingConstructorParams);
    let Ok(five_bytes) = decode(SCALAR_FIVE_HEX) else {
        panic!("the vector decodes")
    };
    let mut uniform = vec![0u8; 63];
    uniform.push(0x05);
    let payload = build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
        uniform: Some(build_secret(SecretConstructorParamsOverrides {
            value: Some(uniform),
        })),
    });

    // Act
    let Ok(sampled) =
        Bls12381ArkworksScalar::sample_from_uniform_bytes(SampleUniformScalarParams, payload)
    else {
        panic!("the uniform input samples")
    };
    let Ok(encoded) = pairing.encode_scalar(
        EncodeScalarParams,
        build_encode_scalar_payload(EncodeScalarPayloadOverrides {
            scalar: Some(sampled.scalar.expose().clone()),
        }),
    );

    // Assert
    assert_eq!(encoded.bytes.expose(), &five_bytes);
}

/// Contract: the sampled branch of `sample_from_uniform_bytes` reduces the
///   input modulo the group order — the largest 64-byte input yields a
///   scalar that decodes canonically.
/// Arrange: the pairing adapter and a uniform payload holding 64 bytes of
///   `0xff`.
/// Act:     `Bls12381ArkworksScalar::sample_from_uniform_bytes` over the
///   payload, then `encode_scalar` of a clone of the exposed scalar, then
///   `decode_scalar` over the encoding.
/// Assert:  the decode returns `Ok` — the sampled scalar is below `r`.
#[test]
fn sample_from_uniform_bytes_reduces_the_largest_input_into_the_scalar_field() {
    // Arrange
    let Ok(pairing) = Bls12381ArkworksPairing::try_new(Bls12381ArkworksPairingConstructorParams);
    let payload = build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
        uniform: Some(build_secret(SecretConstructorParamsOverrides {
            value: Some(vec![0xffu8; 64]),
        })),
    });

    // Act
    let Ok(sampled) =
        Bls12381ArkworksScalar::sample_from_uniform_bytes(SampleUniformScalarParams, payload)
    else {
        panic!("the uniform input samples")
    };
    let Ok(encoded) = pairing.encode_scalar(
        EncodeScalarParams,
        build_encode_scalar_payload(EncodeScalarPayloadOverrides {
            scalar: Some(sampled.scalar.expose().clone()),
        }),
    );
    let result = pairing.decode_scalar(DecodeScalarParams, encoded.bytes.expose());

    // Assert
    assert!(result.is_ok());
}

/// Contract: the sampled branch of `sample_from_uniform_bytes` over a
///   production draw yields a scalar that decodes canonically.
/// Arrange: the pairing adapter, the operating-system random source, and a
///   draw of `UNIFORM_BYTES_LENGTH` bytes.
/// Act:     `fill_bytes` over the source, then
///   `Bls12381ArkworksScalar::sample_from_uniform_bytes` over the draw, then
///   `encode_scalar` and `decode_scalar`.
/// Assert:  the decode returns `Ok` — the sampled scalar is below `r`.
#[test]
fn sample_from_uniform_bytes_draws_an_in_range_scalar_from_the_operating_system_source() {
    // Arrange
    let Ok(pairing) = Bls12381ArkworksPairing::try_new(Bls12381ArkworksPairingConstructorParams);
    let Ok(created) = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    );
    let Ok(drawn) = created.adapter.fill_bytes(
        FillBytesParams,
        build_fill_bytes_payload(FillBytesPayloadOverrides {
            length: Some(Bls12381ArkworksScalar::UNIFORM_BYTES_LENGTH),
        }),
    ) else {
        panic!("the source draws")
    };
    let payload = build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
        uniform: Some(drawn.bytes),
    });

    // Act
    let Ok(sampled) =
        Bls12381ArkworksScalar::sample_from_uniform_bytes(SampleUniformScalarParams, payload)
    else {
        panic!("the draw samples")
    };
    let Ok(encoded) = pairing.encode_scalar(
        EncodeScalarParams,
        build_encode_scalar_payload(EncodeScalarPayloadOverrides {
            scalar: Some(sampled.scalar.expose().clone()),
        }),
    );
    let result = pairing.decode_scalar(DecodeScalarParams, encoded.bytes.expose());

    // Assert
    assert!(result.is_ok());
}

/// Contract: the concrete's declaration names its curve, the verifier's
///   group arithmetic, its precompile encoding, its adapter version, and
///   the interface version it implements, readable from the type before any
///   instance exists.
/// Arrange: nothing.
/// Act:     read `Bls12381ArkworksPairing::DECLARATION`.
/// Assert:  `curve` is `PairingCurve::Bls12381`, `verifier_group_arithmetic`
///   is `VerifierGroupArithmetic::BothGroups`, `precompile_encoding` is
///   `PrecompileEncoding::Eip2537`, `adapter_version` equals 1, and
///   `interface_version` equals `PAIRING_INTERFACE_VERSION`.
#[test]
fn bls12_381_arkworks_pairing_declares_its_curve_arithmetic_encoding_and_versions() {
    // Arrange

    // Act
    let declaration = Bls12381ArkworksPairing::DECLARATION;

    // Assert
    assert!(matches!(declaration.curve, PairingCurve::Bls12381));
    assert!(matches!(
        declaration.verifier_group_arithmetic,
        VerifierGroupArithmetic::BothGroups
    ));
    assert!(matches!(
        declaration.precompile_encoding,
        PrecompileEncoding::Eip2537
    ));
    assert_eq!(declaration.adapter_version, 1);
    assert_eq!(declaration.interface_version, PAIRING_INTERFACE_VERSION);
}
