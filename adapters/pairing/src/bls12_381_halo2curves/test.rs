#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    Bls12381Halo2curvesPairing, Bls12381Halo2curvesPairingConstructorParams,
    Bls12381Halo2curvesScalar,
};
use crate::factory::provides::{
    AddG1Params, AddG1PayloadOverrides, AddG2Params, AddG2PayloadOverrides, AddScalarParams,
    AddScalarPayloadOverrides, DecodeG1ErrorReturn, DecodeG1Params, DecodeG2ErrorReturn,
    DecodeG2Params, DecodeScalarErrorReturn, DecodeScalarParams, EncodeG1Params,
    EncodeG1PayloadOverrides, EncodeG2Params, EncodeG2PayloadOverrides, EncodeGtParams,
    EncodeGtPayloadOverrides, EncodeScalarParams, EncodeScalarPayloadOverrides, G1GeneratorParams,
    G1GeneratorPayload, G2GeneratorParams, G2GeneratorPayload, IPairingAdapter, IPairingArithmetic,
    ISampleUniformScalar, IsIdentityG1Params, IsIdentityG1PayloadOverrides, IsIdentityG2Params,
    IsIdentityG2PayloadOverrides, MsmG1Params, MsmG1PayloadOverrides, MsmG1TermOverrides,
    MsmG2Params, MsmG2PayloadOverrides, MsmG2TermOverrides, MulG1Params, MulG1PayloadOverrides,
    MulG2Params, MulG2PayloadOverrides, MulScalarParams, MulScalarPayloadOverrides, NegG1Params,
    NegG1PayloadOverrides, NegG2Params, NegG2PayloadOverrides, NegScalarParams,
    NegScalarPayloadOverrides, PAIRING_INTERFACE_VERSION, PairingCurve, PairingProductIsOneParams,
    PairingProductIsOnePayloadOverrides, PairingProductParams, PairingProductPayloadOverrides,
    PairingProductTermOverrides, PrecompileEncoding, SampleUniformScalarErrorReturn,
    SampleUniformScalarParams, SampleUniformScalarPayloadOverrides, TargetGroupEncodingIdentifier,
    VerifierGroupArithmetic, build_add_g1_payload, build_add_g2_payload, build_add_scalar_payload,
    build_encode_g1_payload, build_encode_g2_payload, build_encode_gt_payload,
    build_encode_scalar_payload, build_is_identity_g1_payload, build_is_identity_g2_payload,
    build_msm_g1_payload, build_msm_g1_term, build_msm_g2_payload, build_msm_g2_term,
    build_mul_g1_payload, build_mul_g2_payload, build_mul_scalar_payload, build_neg_g1_payload,
    build_neg_g2_payload, build_neg_scalar_payload, build_pairing_product_is_one_payload,
    build_pairing_product_payload, build_pairing_product_term, build_sample_uniform_scalar_payload,
};
use core::iter::successors;
use domain::{SecretConstructorParamsOverrides, build_secret};
use halo2curves::CurveAffine;
use halo2curves::bls12381::{Bls12381, Fq, Fq2, Fq12, Fr, G1Affine, G2Affine};
use halo2curves::ff::{Field, PrimeField};
use halo2curves::group::{cofactor::CofactorGroup, prime::PrimeCurveAffine};
use halo2curves::pairing::MultiMillerLoop;
use hex::decode;
use num_bigint::BigUint;
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
const SCALAR_ZERO_HEX: &str = "0000000000000000000000000000000000000000000000000000000000000000";
const SCALAR_ONE_HEX: &str = "0000000000000000000000000000000000000000000000000000000000000001";
const SCALAR_TWO_HEX: &str = "0000000000000000000000000000000000000000000000000000000000000002";
const SCALAR_THREE_HEX: &str = "0000000000000000000000000000000000000000000000000000000000000003";
const SCALAR_FIVE_HEX: &str = "0000000000000000000000000000000000000000000000000000000000000005";
const SCALAR_SIX_HEX: &str = "0000000000000000000000000000000000000000000000000000000000000006";
const NEG_G1_GENERATOR_HEX: &str = "00000000000000000000000000000000\
     17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb\
     00000000000000000000000000000000\
     114d1d6855d545a8aa7d76c8cf2e21f267816aef1db507c96655b9d5caac42364e6f38ba0ecb751bad54dcd6b939c2ca";
const CFRG_GENERATOR_PAIRING_HEX: &str = "11619b45f61edfe3b47a15fac19442526ff489dcda25e59121d9931438907dfd448299a87dde3a649bdba96e84d54558\
     153ce14a76a53e205ba8f275ef1137c56a566f638b52d34ba3bf3bf22f277d70f76316218c0dfd583a394b8448d2be7f\
     095668fb4a02fe930ed44767834c915b283b1c6ca98c047bd4c272e9ac3f3ba6ff0b05a93e59c71fba77bce995f04692\
     16deedaa683124fe7260085184d88f7d036b86f53bb5b7f1fc5e248814782065413e7d958d17960109ea006b2afdeb5f\
     09c92cf02f3cd3d2f9d34bc44eee0dd50314ed44ca5d30ce6a9ec0539be7a86b121edc61839ccc908c4bdde256cd6048\
     111061f398efc2a97ff825b04d21089e24fd8b93a47e41e60eae7e9b2a38d54fa4dedced0811c34ce528781ab9e929c7\
     01ecfcf31c86257ab00b4709c33f1c9c4e007659dd5ffc4a735192167ce197058cfb4c94225e7f1b6c26ad9ba68f63bc\
     08890726743a1f94a8193a166800b7787744a8ad8e2f9365db76863e894b7a11d83f90d873567e9d645ccf725b32d26f\
     0e61c752414ca5dfd258e9606bac08daec29b3e2c57062669556954fb227d3f1260eedf25446a086b0844bcd43646c10\
     0fe63f185f56dd29150fc498bbeea78969e7e783043620db33f75a05a0a2ce5c442beaff9da195ff15164c00ab66bdde\
     10900338a92ed0b47af211636f7cfdec717b7ee43900eee9b5fc24f0000c5874d4801372db478987691c566a8c474978\
     1454814f3085f0e6602247671bc408bbce2007201536818c901dbd4d2095dd86c1ec8b888e59611f60a301af7776be3d";

fn definition_exponent() -> Vec<u64> {
    let Some(p) = BigUint::parse_bytes(Fq::MODULUS.trim_start_matches("0x").as_bytes(), 16) else {
        panic!("the base field modulus parses")
    };
    let Some(r) = BigUint::parse_bytes(Fr::MODULUS.trim_start_matches("0x").as_bytes(), 16) else {
        panic!("the group order parses")
    };
    ((p.pow(12) - BigUint::from(1u32)) / r).to_u64_digits()
}

fn tower_bytes(value: &Fq12) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(576);
    for coefficient in [
        value.c0().c0().c0(),
        value.c0().c0().c1(),
        value.c0().c1().c0(),
        value.c0().c1().c1(),
        value.c0().c2().c0(),
        value.c0().c2().c1(),
        value.c1().c0().c0(),
        value.c1().c0().c1(),
        value.c1().c1().c0(),
        value.c1().c1().c1(),
        value.c1().c2().c0(),
        value.c1().c2().c1(),
    ] {
        bytes.extend(coefficient.to_repr().as_ref().iter().rev());
    }
    bytes
}

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
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
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
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
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
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
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
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
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
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
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
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
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

/// Contract: the encoded branch of `encode_g1` — a point whose `coordinates()`
///   is none — writes the identity as 128 zero bytes.
/// Arrange: the pairing adapter and the identity produced by `msm_g1` over an
///   empty term list.
/// Act:     `pairing.msm_g1` over the empty payload, then `pairing.encode_g1`.
/// Assert:  the encoded bytes equal 128 zero bytes.
#[test]
fn encode_g1_writes_the_identity_as_all_zero_bytes() {
    // Arrange
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
    let expected = vec![0u8; 128];

    // Act
    let Ok(identity) = pairing.msm_g1(
        MsmG1Params,
        build_msm_g1_payload(MsmG1PayloadOverrides::default()),
    );
    let Ok(encoded) = pairing.encode_g1(
        EncodeG1Params,
        build_encode_g1_payload(EncodeG1PayloadOverrides {
            point: Some(identity.sum),
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
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
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
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
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
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
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
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
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
///   subgroup found by ascending `x`, `y` from the square root of
///   `x^3 + b`, encoded as `x` then `y`, each 16 zero bytes followed by its
///   48 big-endian bytes.
/// Act:     `pairing.decode_g1` over the encoded point.
/// Assert:  the return is `Err(DecodeG1ErrorReturn::NotInSubgroup)`.
#[test]
fn decode_g1_rejects_a_point_outside_the_subgroup() {
    // Arrange
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
    let Some((x, y)) = successors(Some(Fq::ONE), |x| Some(*x + Fq::ONE)).find_map(|x| {
        Option::<Fq>::from((x.square() * x + G1Affine::b()).sqrt()).and_then(|y| {
            Option::<G1Affine>::from(G1Affine::from_xy(x, y))
                .filter(|point| !bool::from(point.to_curve().is_torsion_free()))
                .map(|_| (x, y))
        })
    }) else {
        panic!("an on-curve point outside the subgroup exists")
    };
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&[0u8; 16]);
    bytes.extend(x.to_repr().as_ref().iter().rev());
    bytes.extend_from_slice(&[0u8; 16]);
    bytes.extend(y.to_repr().as_ref().iter().rev());

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
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
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
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
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
///   subgroup found by ascending `x.c0` over `Fq2::new(c0, Fq::ZERO)`, `y`
///   from the square root of `x^3 + b`, encoded as `x.c0`, `x.c1`, `y.c0`,
///   `y.c1`, each 16 zero bytes followed by its 48 big-endian bytes.
/// Act:     `pairing.decode_g2` over the encoded point.
/// Assert:  the return is `Err(DecodeG2ErrorReturn::NotInSubgroup)`.
#[test]
fn decode_g2_rejects_a_point_outside_the_subgroup() {
    // Arrange
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
    let Some((x, y)) = successors(Some(Fq::ONE), |c0| Some(*c0 + Fq::ONE)).find_map(|c0| {
        let x = Fq2::new(c0, Fq::ZERO);
        Option::<Fq2>::from((x.square() * x + G2Affine::b()).sqrt()).and_then(|y| {
            Option::<G2Affine>::from(G2Affine::from_xy(x, y))
                .filter(|point| !bool::from(point.to_curve().is_torsion_free()))
                .map(|_| (x, y))
        })
    }) else {
        panic!("an on-curve point outside the subgroup exists")
    };
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&[0u8; 16]);
    bytes.extend(x.c0().to_repr().as_ref().iter().rev());
    bytes.extend_from_slice(&[0u8; 16]);
    bytes.extend(x.c1().to_repr().as_ref().iter().rev());
    bytes.extend_from_slice(&[0u8; 16]);
    bytes.extend(y.c0().to_repr().as_ref().iter().rev());
    bytes.extend_from_slice(&[0u8; 16]);
    bytes.extend(y.c1().to_repr().as_ref().iter().rev());

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
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
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
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
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
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
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
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
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
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
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
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
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
/// Arrange: the pairing adapter, the second-group generator, and the
///   decoded scalars two, three, and five.
/// Act:     `pairing.msm_g2` over terms (g2, 2) and (g2, 3), and
///   `pairing.mul_g2` of the generator and five, each encoded.
/// Assert:  the encodings are equal.
#[test]
fn msm_g2_equals_the_multiple_by_the_sum_of_its_scalars() {
    // Arrange
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
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
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
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
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
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
///   moved bilinearly between the two source groups — returns `is_one: true`
///   for `(g1 · 2, g2)` with `(g1, g2 · (r - 2))`.
/// Arrange: the pairing adapter, both generators, and the decoded scalars
///   two and `r - 2`.
/// Act:     `pairing.pairing_product_is_one` over the terms.
/// Assert:  `is_one` is `true`.
#[test]
fn pairing_product_is_one_across_the_bilinear_exchange() {
    // Arrange
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
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
/// Act:     `Bls12381Halo2curvesScalar::sample_from_uniform_bytes` over the
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
        Bls12381Halo2curvesScalar::sample_from_uniform_bytes(SampleUniformScalarParams, payload);

    // Assert
    assert!(matches!(
        result,
        Err(SampleUniformScalarErrorReturn::WrongLength {
            expected: 64,
            actual: 63
        })
    ));
}

/// Contract: the sampled branch of `sample_from_uniform_bytes` reads its
///   input as one big-endian integer reduced modulo the group order — 63
///   zero bytes followed by `05` samples the scalar five.
/// Arrange: the pairing adapter and a uniform payload of 63 zero bytes and
///   `05`.
/// Act:     `Bls12381Halo2curvesScalar::sample_from_uniform_bytes` over the
///   payload, then `encode_scalar` of a clone of the exposed scalar.
/// Assert:  the exposed encoded bytes equal the 32-byte big-endian encoding
///   of five.
#[test]
fn sample_from_uniform_bytes_reads_its_input_as_a_big_endian_integer() {
    // Arrange
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
    let Ok(expected) = decode(SCALAR_FIVE_HEX) else {
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
        Bls12381Halo2curvesScalar::sample_from_uniform_bytes(SampleUniformScalarParams, payload)
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
    assert_eq!(encoded.bytes.expose(), &expected);
}

/// Contract: the sampled branch of `sample_from_uniform_bytes` reduces the
///   input modulo the group order — the largest 64-byte input yields a
///   scalar that decodes canonically.
/// Arrange: the pairing adapter and a uniform payload holding 64 bytes of
///   `0xff`.
/// Act:     `Bls12381Halo2curvesScalar::sample_from_uniform_bytes` over the
///   payload, then `encode_scalar` of a clone of the exposed scalar, then
///   `decode_scalar` over the encoding.
/// Assert:  the decode returns `Ok` — the sampled scalar is below `r`.
#[test]
fn sample_from_uniform_bytes_reduces_the_largest_input_into_the_scalar_field() {
    // Arrange
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
    let payload = build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
        uniform: Some(build_secret(SecretConstructorParamsOverrides {
            value: Some(vec![0xffu8; 64]),
        })),
    });

    // Act
    let Ok(sampled) =
        Bls12381Halo2curvesScalar::sample_from_uniform_bytes(SampleUniformScalarParams, payload)
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
///   `Bls12381Halo2curvesScalar::sample_from_uniform_bytes` over the draw,
///   then `encode_scalar` and `decode_scalar`.
/// Assert:  the decode returns `Ok` — the sampled scalar is below `r`.
#[test]
fn sample_from_uniform_bytes_draws_an_in_range_scalar_from_the_operating_system_source() {
    // Arrange
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
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
            length: Some(Bls12381Halo2curvesScalar::UNIFORM_BYTES_LENGTH),
        }),
    ) else {
        panic!("the source draws")
    };
    let payload = build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
        uniform: Some(drawn.bytes),
    });

    // Act
    let Ok(sampled) =
        Bls12381Halo2curvesScalar::sample_from_uniform_bytes(SampleUniformScalarParams, payload)
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
/// Act:     read `Bls12381Halo2curvesPairing::DECLARATION`.
/// Assert:  `curve` is `PairingCurve::Bls12381`, `verifier_group_arithmetic`
///   is `VerifierGroupArithmetic::BothGroups`, `precompile_encoding` is
///   `PrecompileEncoding::Eip2537`, `adapter_version` equals 1, and
///   `interface_version` equals `PAIRING_INTERFACE_VERSION`.
#[test]
fn bls12_381_halo2curves_pairing_declares_its_curve_arithmetic_encoding_and_versions() {
    // Arrange

    // Act
    let declaration = Bls12381Halo2curvesPairing::DECLARATION;

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

/// Contract: the summed branch of `add_scalar` returns the operands' sum in
///   the scalar field — two plus three is five.
/// Arrange: the pairing adapter and the decoded scalars two and three.
/// Act:     `pairing.add_scalar` over the payload, then `pairing.encode_scalar`.
/// Assert:  the exposed encoded bytes equal the scalar five.
#[test]
fn add_scalar_of_two_and_three_is_five() {
    // Arrange
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
    let Ok(two_bytes) = decode(SCALAR_TWO_HEX) else {
        panic!("the vector decodes")
    };
    let Ok(three_bytes) = decode(SCALAR_THREE_HEX) else {
        panic!("the vector decodes")
    };
    let Ok(five_bytes) = decode(SCALAR_FIVE_HEX) else {
        panic!("the vector decodes")
    };
    let Ok(two) = pairing.decode_scalar(DecodeScalarParams, &two_bytes) else {
        panic!("the scalar decodes")
    };
    let Ok(three) = pairing.decode_scalar(DecodeScalarParams, &three_bytes) else {
        panic!("the scalar decodes")
    };

    // Act
    let Ok(summed) = pairing.add_scalar(
        AddScalarParams,
        build_add_scalar_payload(AddScalarPayloadOverrides {
            left: Some(two.scalar),
            right: Some(three.scalar),
        }),
    );
    let Ok(encoded) = pairing.encode_scalar(
        EncodeScalarParams,
        build_encode_scalar_payload(EncodeScalarPayloadOverrides {
            scalar: Some(summed.sum),
        }),
    );

    // Assert
    assert_eq!(encoded.bytes.expose(), &five_bytes);
}

/// Contract: the summed branch of `add_scalar` reduces modulo the group order
///   — `r - 1` plus two is one.
/// Arrange: the pairing adapter and the decoded scalars `r - 1` and two.
/// Act:     `pairing.add_scalar` over the payload, then `pairing.encode_scalar`.
/// Assert:  the exposed encoded bytes equal the scalar one.
#[test]
fn add_scalar_reduces_modulo_the_group_order() {
    // Arrange
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
    let Ok(r_minus_one_bytes) = decode(GROUP_ORDER_MINUS_ONE_HEX) else {
        panic!("the vector decodes")
    };
    let Ok(two_bytes) = decode(SCALAR_TWO_HEX) else {
        panic!("the vector decodes")
    };
    let Ok(one_bytes) = decode(SCALAR_ONE_HEX) else {
        panic!("the vector decodes")
    };
    let Ok(r_minus_one) = pairing.decode_scalar(DecodeScalarParams, &r_minus_one_bytes) else {
        panic!("the scalar decodes")
    };
    let Ok(two) = pairing.decode_scalar(DecodeScalarParams, &two_bytes) else {
        panic!("the scalar decodes")
    };

    // Act
    let Ok(summed) = pairing.add_scalar(
        AddScalarParams,
        build_add_scalar_payload(AddScalarPayloadOverrides {
            left: Some(r_minus_one.scalar),
            right: Some(two.scalar),
        }),
    );
    let Ok(encoded) = pairing.encode_scalar(
        EncodeScalarParams,
        build_encode_scalar_payload(EncodeScalarPayloadOverrides {
            scalar: Some(summed.sum),
        }),
    );

    // Assert
    assert_eq!(encoded.bytes.expose(), &one_bytes);
}

/// Contract: the multiplied branch of `mul_scalar` returns the operands'
///   product in the scalar field — two times three is six.
/// Arrange: the pairing adapter and the decoded scalars two and three.
/// Act:     `pairing.mul_scalar` over the payload, then `pairing.encode_scalar`.
/// Assert:  the exposed encoded bytes equal the scalar six.
#[test]
fn mul_scalar_of_two_and_three_is_six() {
    // Arrange
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
    let Ok(two_bytes) = decode(SCALAR_TWO_HEX) else {
        panic!("the vector decodes")
    };
    let Ok(three_bytes) = decode(SCALAR_THREE_HEX) else {
        panic!("the vector decodes")
    };
    let Ok(six_bytes) = decode(SCALAR_SIX_HEX) else {
        panic!("the vector decodes")
    };
    let Ok(two) = pairing.decode_scalar(DecodeScalarParams, &two_bytes) else {
        panic!("the scalar decodes")
    };
    let Ok(three) = pairing.decode_scalar(DecodeScalarParams, &three_bytes) else {
        panic!("the scalar decodes")
    };

    // Act
    let Ok(multiplied) = pairing.mul_scalar(
        MulScalarParams,
        build_mul_scalar_payload(MulScalarPayloadOverrides {
            left: Some(two.scalar),
            right: Some(three.scalar),
        }),
    );
    let Ok(encoded) = pairing.encode_scalar(
        EncodeScalarParams,
        build_encode_scalar_payload(EncodeScalarPayloadOverrides {
            scalar: Some(multiplied.product),
        }),
    );

    // Assert
    assert_eq!(encoded.bytes.expose(), &six_bytes);
}

/// Contract: the multiplied branch of `mul_scalar` reduces modulo the group
///   order — `(r - 1)^2` is one modulo `r`.
/// Arrange: the pairing adapter and the decoded scalar `r - 1`.
/// Act:     `pairing.mul_scalar` over the payload, then `pairing.encode_scalar`.
/// Assert:  the exposed encoded bytes equal the scalar one.
#[test]
fn mul_scalar_reduces_modulo_the_group_order() {
    // Arrange
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
    let Ok(r_minus_one_bytes) = decode(GROUP_ORDER_MINUS_ONE_HEX) else {
        panic!("the vector decodes")
    };
    let Ok(one_bytes) = decode(SCALAR_ONE_HEX) else {
        panic!("the vector decodes")
    };
    let Ok(r_minus_one) = pairing.decode_scalar(DecodeScalarParams, &r_minus_one_bytes) else {
        panic!("the scalar decodes")
    };

    // Act
    let Ok(multiplied) = pairing.mul_scalar(
        MulScalarParams,
        build_mul_scalar_payload(MulScalarPayloadOverrides {
            left: Some(r_minus_one.scalar.clone()),
            right: Some(r_minus_one.scalar),
        }),
    );
    let Ok(encoded) = pairing.encode_scalar(
        EncodeScalarParams,
        build_encode_scalar_payload(EncodeScalarPayloadOverrides {
            scalar: Some(multiplied.product),
        }),
    );

    // Assert
    assert_eq!(encoded.bytes.expose(), &one_bytes);
}

/// Contract: the negated branch of `neg_scalar` returns the group order minus
///   the scalar — the negation of one is `r - 1`.
/// Arrange: the pairing adapter and the decoded scalar one.
/// Act:     `pairing.neg_scalar` over the payload, then `pairing.encode_scalar`.
/// Assert:  the exposed encoded bytes equal `r - 1`.
#[test]
fn neg_scalar_of_one_is_the_group_order_minus_one() {
    // Arrange
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
    let Ok(one_bytes) = decode(SCALAR_ONE_HEX) else {
        panic!("the vector decodes")
    };
    let Ok(r_minus_one_bytes) = decode(GROUP_ORDER_MINUS_ONE_HEX) else {
        panic!("the vector decodes")
    };
    let Ok(one) = pairing.decode_scalar(DecodeScalarParams, &one_bytes) else {
        panic!("the scalar decodes")
    };

    // Act
    let Ok(negated) = pairing.neg_scalar(
        NegScalarParams,
        build_neg_scalar_payload(NegScalarPayloadOverrides {
            scalar: Some(one.scalar),
        }),
    );
    let Ok(encoded) = pairing.encode_scalar(
        EncodeScalarParams,
        build_encode_scalar_payload(EncodeScalarPayloadOverrides {
            scalar: Some(negated.negation),
        }),
    );

    // Assert
    assert_eq!(encoded.bytes.expose(), &r_minus_one_bytes);
}

/// Contract: the negated branch of `neg_scalar` over zero is zero.
/// Arrange: the pairing adapter and the decoded scalar zero.
/// Act:     `pairing.neg_scalar` over the payload, then `pairing.encode_scalar`.
/// Assert:  the exposed encoded bytes equal the scalar zero.
#[test]
fn neg_scalar_of_zero_is_zero() {
    // Arrange
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
    let Ok(zero_bytes) = decode(SCALAR_ZERO_HEX) else {
        panic!("the vector decodes")
    };
    let Ok(zero) = pairing.decode_scalar(DecodeScalarParams, &zero_bytes) else {
        panic!("the scalar decodes")
    };

    // Act
    let Ok(negated) = pairing.neg_scalar(
        NegScalarParams,
        build_neg_scalar_payload(NegScalarPayloadOverrides {
            scalar: Some(zero.scalar),
        }),
    );
    let Ok(encoded) = pairing.encode_scalar(
        EncodeScalarParams,
        build_encode_scalar_payload(EncodeScalarPayloadOverrides {
            scalar: Some(negated.negation),
        }),
    );

    // Assert
    assert_eq!(encoded.bytes.expose(), &zero_bytes);
}

/// Contract: the negated branch of `neg_g1` keeps `x` and replaces `y` by
///   `p - y` — the generator's negation is the EIP-2537 encoding of the
///   generator's `x` with `p - y`.
/// Arrange: the pairing adapter, the first-group generator, and its negated
///   encoding.
/// Act:     `pairing.neg_g1` over the payload, then `pairing.encode_g1`.
/// Assert:  the encoded bytes equal the negated generator vector.
#[test]
fn neg_g1_of_the_generator_negates_its_y_coordinate() {
    // Arrange
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
    let Ok(expected) = decode(NEG_G1_GENERATOR_HEX) else {
        panic!("the vector decodes")
    };
    let Ok(generated) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);

    // Act
    let Ok(negated) = pairing.neg_g1(
        NegG1Params,
        build_neg_g1_payload(NegG1PayloadOverrides {
            point: Some(generated.point),
        }),
    );
    let Ok(encoded) = pairing.encode_g1(
        EncodeG1Params,
        build_encode_g1_payload(EncodeG1PayloadOverrides {
            point: Some(negated.negation),
        }),
    );

    // Assert
    assert_eq!(encoded.bytes, expected);
}

/// Contract: the negated branch of `neg_g1` over the identity is the
///   identity.
/// Arrange: the pairing adapter and the point `decode_g1` reads from 128 zero
///   bytes.
/// Act:     `pairing.neg_g1` over the payload, then `pairing.encode_g1`.
/// Assert:  the encoded bytes are 128 zero bytes.
#[test]
fn neg_g1_of_the_identity_is_the_identity() {
    // Arrange
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
    let zeros = vec![0u8; 128];
    let Ok(identity) = pairing.decode_g1(DecodeG1Params, &zeros) else {
        panic!("the all-zero encoding decodes to the identity")
    };

    // Act
    let Ok(negated) = pairing.neg_g1(
        NegG1Params,
        build_neg_g1_payload(NegG1PayloadOverrides {
            point: Some(identity.point),
        }),
    );
    let Ok(encoded) = pairing.encode_g1(
        EncodeG1Params,
        build_encode_g1_payload(EncodeG1PayloadOverrides {
            point: Some(negated.negation),
        }),
    );

    // Assert
    assert_eq!(encoded.bytes, zeros);
}

/// Contract: a second-group point plus its negation is the identity — the
///   generator and the negated branch of `neg_g2` sum to the identity.
/// Arrange: the pairing adapter and the second-group generator.
/// Act:     `pairing.neg_g2` over the generator, `pairing.add_g2` of the
///   generator and the negation, then `pairing.encode_g2`.
/// Assert:  the encoded bytes are 256 zero bytes.
#[test]
fn neg_g2_of_the_generator_sums_with_the_generator_to_the_identity() {
    // Arrange
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
    let zeros = vec![0u8; 256];
    let Ok(generated) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);

    // Act
    let Ok(negated) = pairing.neg_g2(
        NegG2Params,
        build_neg_g2_payload(NegG2PayloadOverrides {
            point: Some(generated.point.clone()),
        }),
    );
    let Ok(added) = pairing.add_g2(
        AddG2Params,
        build_add_g2_payload(AddG2PayloadOverrides {
            left: Some(generated.point),
            right: Some(negated.negation),
        }),
    );
    let Ok(encoded) = pairing.encode_g2(
        EncodeG2Params,
        build_encode_g2_payload(EncodeG2PayloadOverrides {
            point: Some(added.sum),
        }),
    );

    // Assert
    assert_eq!(encoded.bytes, zeros);
}

/// Contract: the tested branch of `is_identity_g1` over the identity reports
///   `true`.
/// Arrange: the pairing adapter and the point `decode_g1` reads from 128 zero
///   bytes.
/// Act:     `pairing.is_identity_g1` over the payload.
/// Assert:  `is_identity` is `true`.
#[test]
fn is_identity_g1_is_true_for_the_identity() {
    // Arrange
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
    let zeros = vec![0u8; 128];
    let Ok(identity) = pairing.decode_g1(DecodeG1Params, &zeros) else {
        panic!("the all-zero encoding decodes to the identity")
    };

    // Act
    let Ok(tested) = pairing.is_identity_g1(
        IsIdentityG1Params,
        build_is_identity_g1_payload(IsIdentityG1PayloadOverrides {
            point: Some(identity.point),
        }),
    );

    // Assert
    assert!(tested.is_identity);
}

/// Contract: the tested branch of `is_identity_g1` over a non-identity point
///   reports `false`.
/// Arrange: the pairing adapter and the first-group generator.
/// Act:     `pairing.is_identity_g1` over the payload.
/// Assert:  `is_identity` is `false`.
#[test]
fn is_identity_g1_is_false_for_the_generator() {
    // Arrange
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
    let Ok(generated) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);

    // Act
    let Ok(tested) = pairing.is_identity_g1(
        IsIdentityG1Params,
        build_is_identity_g1_payload(IsIdentityG1PayloadOverrides {
            point: Some(generated.point),
        }),
    );

    // Assert
    assert!(!tested.is_identity);
}

/// Contract: the tested branch of `is_identity_g2` over the identity reports
///   `true`.
/// Arrange: the pairing adapter and the point `decode_g2` reads from 256 zero
///   bytes.
/// Act:     `pairing.is_identity_g2` over the payload.
/// Assert:  `is_identity` is `true`.
#[test]
fn is_identity_g2_is_true_for_the_identity() {
    // Arrange
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
    let zeros = vec![0u8; 256];
    let Ok(identity) = pairing.decode_g2(DecodeG2Params, &zeros) else {
        panic!("the all-zero encoding decodes to the identity")
    };

    // Act
    let Ok(tested) = pairing.is_identity_g2(
        IsIdentityG2Params,
        build_is_identity_g2_payload(IsIdentityG2PayloadOverrides {
            point: Some(identity.point),
        }),
    );

    // Assert
    assert!(tested.is_identity);
}

/// Contract: the tested branch of `is_identity_g2` over a non-identity point
///   reports `false`.
/// Arrange: the pairing adapter and the second-group generator.
/// Act:     `pairing.is_identity_g2` over the payload.
/// Assert:  `is_identity` is `false`.
#[test]
fn is_identity_g2_is_false_for_the_generator() {
    // Arrange
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
    let Ok(generated) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);

    // Act
    let Ok(tested) = pairing.is_identity_g2(
        IsIdentityG2Params,
        build_is_identity_g2_payload(IsIdentityG2PayloadOverrides {
            point: Some(generated.point),
        }),
    );

    // Assert
    assert!(!tested.is_identity);
}

/// Contract: the evaluated branch of `pairing_product` over no terms is the
///   target group's identity, which encodes with the coefficient `c0.c0.c0`
///   first — 47 zero bytes, `01`, then 528 zero bytes.
/// Arrange: the pairing adapter, `build_pairing_product_payload` with its
///   default empty terms, and the identity encoding.
/// Act:     `pairing.pairing_product` over the payload, then
///   `pairing.encode_gt` over the product.
/// Assert:  the exposed encoded bytes equal the identity encoding.
#[test]
fn pairing_product_of_no_terms_encodes_as_the_target_group_identity() {
    // Arrange
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
    let mut identity = vec![0u8; 576];
    identity[47] = 1;

    // Act
    let Ok(product) = pairing.pairing_product(
        PairingProductParams,
        build_pairing_product_payload(PairingProductPayloadOverrides::default()),
    );
    let Ok(encoded) = pairing.encode_gt(
        EncodeGtParams,
        build_encode_gt_payload(EncodeGtPayloadOverrides {
            value: Some(product.product),
        }),
    );

    // Assert
    assert_eq!(encoded.bytes.expose(), &identity);
}

/// Contract: the pairing is non-degenerate — the evaluated branch of
///   `pairing_product` over `(g1, g2)` is not the target group's identity.
/// Arrange: the pairing adapter, both generators, the one term `(g1, g2)`,
///   and the identity encoding.
/// Act:     `pairing.pairing_product` over the term, then `pairing.encode_gt`
///   over the product.
/// Assert:  the exposed encoded bytes are 576 bytes and differ from the
///   identity encoding.
#[test]
fn pairing_product_of_the_generators_is_not_the_target_group_identity() {
    // Arrange
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
    let mut identity = vec![0u8; 576];
    identity[47] = 1;
    let Ok(g1) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
    let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
    let terms = vec![build_pairing_product_term(PairingProductTermOverrides {
        g1: Some(g1.point),
        g2: Some(g2.point),
    })];

    // Act
    let Ok(product) = pairing.pairing_product(
        PairingProductParams,
        build_pairing_product_payload(PairingProductPayloadOverrides { terms: Some(terms) }),
    );
    let Ok(encoded) = pairing.encode_gt(
        EncodeGtParams,
        build_encode_gt_payload(EncodeGtPayloadOverrides {
            value: Some(product.product),
        }),
    );

    // Assert
    assert_eq!(encoded.bytes.expose().len(), 576);
    assert_ne!(encoded.bytes.expose(), &identity);
}

/// Contract: the evaluated branch of `pairing_product` is bilinear — a scalar
///   moves between the arguments, so `(g1 · 2, g2 · 3)` equals `(g1 · 6, g2)`
///   and differs from `(g1 · 5, g2)`.
/// Arrange: the pairing adapter, both generators, and the decoded scalars
///   two, three, five, and six.
/// Act:     `pairing.pairing_product` over each single-term list, then
///   `pairing.encode_gt` over each product.
/// Assert:  the first two encodings are equal and differ from the third.
#[test]
fn pairing_product_is_bilinear() {
    // Arrange
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
    let Ok(two_bytes) = decode(SCALAR_TWO_HEX) else {
        panic!("the vector decodes")
    };
    let Ok(three_bytes) = decode(SCALAR_THREE_HEX) else {
        panic!("the vector decodes")
    };
    let Ok(five_bytes) = decode(SCALAR_FIVE_HEX) else {
        panic!("the vector decodes")
    };
    let Ok(six_bytes) = decode(SCALAR_SIX_HEX) else {
        panic!("the vector decodes")
    };
    let Ok(g1) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
    let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
    let Ok(two) = pairing.decode_scalar(DecodeScalarParams, &two_bytes) else {
        panic!("the scalar decodes")
    };
    let Ok(three) = pairing.decode_scalar(DecodeScalarParams, &three_bytes) else {
        panic!("the scalar decodes")
    };
    let Ok(five) = pairing.decode_scalar(DecodeScalarParams, &five_bytes) else {
        panic!("the scalar decodes")
    };
    let Ok(six) = pairing.decode_scalar(DecodeScalarParams, &six_bytes) else {
        panic!("the scalar decodes")
    };
    let Ok(g1_times_two) = pairing.mul_g1(
        MulG1Params,
        build_mul_g1_payload(MulG1PayloadOverrides {
            point: Some(g1.point.clone()),
            scalar: Some(two.scalar),
        }),
    );
    let Ok(g2_times_three) = pairing.mul_g2(
        MulG2Params,
        build_mul_g2_payload(MulG2PayloadOverrides {
            point: Some(g2.point.clone()),
            scalar: Some(three.scalar),
        }),
    );
    let Ok(g1_times_six) = pairing.mul_g1(
        MulG1Params,
        build_mul_g1_payload(MulG1PayloadOverrides {
            point: Some(g1.point.clone()),
            scalar: Some(six.scalar),
        }),
    );
    let Ok(g1_times_five) = pairing.mul_g1(
        MulG1Params,
        build_mul_g1_payload(MulG1PayloadOverrides {
            point: Some(g1.point),
            scalar: Some(five.scalar),
        }),
    );

    // Act
    let Ok(first) = pairing.pairing_product(
        PairingProductParams,
        build_pairing_product_payload(PairingProductPayloadOverrides {
            terms: Some(vec![build_pairing_product_term(
                PairingProductTermOverrides {
                    g1: Some(g1_times_two.product),
                    g2: Some(g2_times_three.product),
                },
            )]),
        }),
    );
    let Ok(second) = pairing.pairing_product(
        PairingProductParams,
        build_pairing_product_payload(PairingProductPayloadOverrides {
            terms: Some(vec![build_pairing_product_term(
                PairingProductTermOverrides {
                    g1: Some(g1_times_six.product),
                    g2: Some(g2.point.clone()),
                },
            )]),
        }),
    );
    let Ok(third) = pairing.pairing_product(
        PairingProductParams,
        build_pairing_product_payload(PairingProductPayloadOverrides {
            terms: Some(vec![build_pairing_product_term(
                PairingProductTermOverrides {
                    g1: Some(g1_times_five.product),
                    g2: Some(g2.point),
                },
            )]),
        }),
    );
    let Ok(first_encoded) = pairing.encode_gt(
        EncodeGtParams,
        build_encode_gt_payload(EncodeGtPayloadOverrides {
            value: Some(first.product),
        }),
    );
    let Ok(second_encoded) = pairing.encode_gt(
        EncodeGtParams,
        build_encode_gt_payload(EncodeGtPayloadOverrides {
            value: Some(second.product),
        }),
    );
    let Ok(third_encoded) = pairing.encode_gt(
        EncodeGtParams,
        build_encode_gt_payload(EncodeGtPayloadOverrides {
            value: Some(third.product),
        }),
    );

    // Assert
    assert_eq!(first_encoded.bytes.expose(), second_encoded.bytes.expose());
    assert_ne!(first_encoded.bytes.expose(), third_encoded.bytes.expose());
}

/// Contract: the evaluated branch of `pairing_product` multiplies its terms —
///   `[(g1, g2), (g1, g2)]` equals `[(g1 · 2, g2)]` and differs from
///   `[(g1, g2)]`.
/// Arrange: the pairing adapter, both generators, and the decoded scalar two.
/// Act:     `pairing.pairing_product` over each term list, then
///   `pairing.encode_gt` over each product.
/// Assert:  the first two encodings are equal and differ from the third.
#[test]
fn pairing_product_multiplies_its_terms() {
    // Arrange
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
    let Ok(two_bytes) = decode(SCALAR_TWO_HEX) else {
        panic!("the vector decodes")
    };
    let Ok(g1) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
    let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
    let Ok(two) = pairing.decode_scalar(DecodeScalarParams, &two_bytes) else {
        panic!("the scalar decodes")
    };
    let Ok(g1_times_two) = pairing.mul_g1(
        MulG1Params,
        build_mul_g1_payload(MulG1PayloadOverrides {
            point: Some(g1.point.clone()),
            scalar: Some(two.scalar),
        }),
    );

    // Act
    let Ok(first) = pairing.pairing_product(
        PairingProductParams,
        build_pairing_product_payload(PairingProductPayloadOverrides {
            terms: Some(vec![
                build_pairing_product_term(PairingProductTermOverrides {
                    g1: Some(g1.point.clone()),
                    g2: Some(g2.point.clone()),
                }),
                build_pairing_product_term(PairingProductTermOverrides {
                    g1: Some(g1.point.clone()),
                    g2: Some(g2.point.clone()),
                }),
            ]),
        }),
    );
    let Ok(second) = pairing.pairing_product(
        PairingProductParams,
        build_pairing_product_payload(PairingProductPayloadOverrides {
            terms: Some(vec![build_pairing_product_term(
                PairingProductTermOverrides {
                    g1: Some(g1_times_two.product),
                    g2: Some(g2.point.clone()),
                },
            )]),
        }),
    );
    let Ok(third) = pairing.pairing_product(
        PairingProductParams,
        build_pairing_product_payload(PairingProductPayloadOverrides {
            terms: Some(vec![build_pairing_product_term(
                PairingProductTermOverrides {
                    g1: Some(g1.point),
                    g2: Some(g2.point),
                },
            )]),
        }),
    );
    let Ok(first_encoded) = pairing.encode_gt(
        EncodeGtParams,
        build_encode_gt_payload(EncodeGtPayloadOverrides {
            value: Some(first.product),
        }),
    );
    let Ok(second_encoded) = pairing.encode_gt(
        EncodeGtParams,
        build_encode_gt_payload(EncodeGtPayloadOverrides {
            value: Some(second.product),
        }),
    );
    let Ok(third_encoded) = pairing.encode_gt(
        EncodeGtParams,
        build_encode_gt_payload(EncodeGtPayloadOverrides {
            value: Some(third.product),
        }),
    );

    // Assert
    assert_eq!(first_encoded.bytes.expose(), second_encoded.bytes.expose());
    assert_ne!(first_encoded.bytes.expose(), third_encoded.bytes.expose());
}

/// Contract: the evaluated branch of `pairing_product` divides by a term when
///   its first-group input is negated — `(g1, g2)` with `(-g1, g2)` yields the
///   target group's identity.
/// Arrange: the pairing adapter, both generators, the negation from
///   `neg_g1`, and the identity encoding.
/// Act:     `pairing.pairing_product` over the two terms, then
///   `pairing.encode_gt` over the product.
/// Assert:  the exposed encoded bytes equal the identity encoding.
#[test]
fn pairing_product_of_a_pairing_and_its_first_group_negation_is_the_target_group_identity() {
    // Arrange
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
    let mut identity = vec![0u8; 576];
    identity[47] = 1;
    let Ok(g1) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
    let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
    let Ok(negated) = pairing.neg_g1(
        NegG1Params,
        build_neg_g1_payload(NegG1PayloadOverrides {
            point: Some(g1.point.clone()),
        }),
    );
    let terms = vec![
        build_pairing_product_term(PairingProductTermOverrides {
            g1: Some(g1.point),
            g2: Some(g2.point.clone()),
        }),
        build_pairing_product_term(PairingProductTermOverrides {
            g1: Some(negated.negation),
            g2: Some(g2.point),
        }),
    ];

    // Act
    let Ok(product) = pairing.pairing_product(
        PairingProductParams,
        build_pairing_product_payload(PairingProductPayloadOverrides { terms: Some(terms) }),
    );
    let Ok(encoded) = pairing.encode_gt(
        EncodeGtParams,
        build_encode_gt_payload(EncodeGtPayloadOverrides {
            value: Some(product.product),
        }),
    );

    // Assert
    assert_eq!(encoded.bytes.expose(), &identity);
}

/// Contract: the concrete declares `TargetGroupEncodingIdentifier::Bls12381V1`,
///   the identifier the suite admits it by.
/// Arrange: nothing.
/// Act:     read `Bls12381Halo2curvesPairing::DECLARATION`.
/// Assert:  `target_group_encoding` is
///   `TargetGroupEncodingIdentifier::Bls12381V1`.
#[test]
fn declaration_names_the_bls12_381_target_group_encoding() {
    // Arrange

    // Act
    let declaration = Bls12381Halo2curvesPairing::DECLARATION;

    // Assert
    assert!(matches!(
        declaration.target_group_encoding,
        TargetGroupEncodingIdentifier::Bls12381V1
    ));
}

/// Contract: the evaluated branch of `pairing_product` returns the
///   identifier's exact value — the Miller loop's value raised to the exact
///   exponent `(p^12 - 1) / r`, not the library's cubed value.
/// Arrange: the pairing adapter, both generators, and the expected encoding
///   computed from the identifier's definition — `multi_miller_loop` over the
///   generators raised to `definition_exponent()`, serialized in tower order.
/// Act:     `pairing.pairing_product` over the one term `(g1, g2)`, then
///   `pairing.encode_gt` over the product.
/// Assert:  the exposed encoded bytes equal `expected`.
#[test]
fn pairing_product_of_the_generators_equals_the_definition() {
    // Arrange
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
    let expected = tower_bytes(
        &Bls12381::multi_miller_loop(&[(&G1Affine::generator(), &G2Affine::generator())])
            .pow_vartime(definition_exponent()),
    );
    let Ok(g1) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
    let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
    let terms = vec![build_pairing_product_term(PairingProductTermOverrides {
        g1: Some(g1.point),
        g2: Some(g2.point),
    })];

    // Act
    let Ok(product) = pairing.pairing_product(
        PairingProductParams,
        build_pairing_product_payload(PairingProductPayloadOverrides { terms: Some(terms) }),
    );
    let Ok(encoded) = pairing.encode_gt(
        EncodeGtParams,
        build_encode_gt_payload(EncodeGtPayloadOverrides {
            value: Some(product.product),
        }),
    );

    // Assert
    assert_eq!(encoded.bytes.expose(), &expected);
}

/// Contract: the evaluated branch of `pairing_product` over `(g1, g2)` encodes
///   to the value the CFRG pairing-friendly-curves draft publishes in its
///   test-vector appendix for the pairing of the base points.
/// Arrange: the pairing adapter, both generators, the one term `(g1, g2)`, and
///   the published 576-byte vector.
/// Act:     `pairing.pairing_product` over the term, then `pairing.encode_gt`
///   over the product.
/// Assert:  the exposed encoded bytes equal the decoded vector.
#[test]
fn pairing_product_of_the_generators_encodes_to_the_published_vector() {
    // Arrange
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
    let Ok(expected) = decode(CFRG_GENERATOR_PAIRING_HEX) else {
        panic!("the vector decodes")
    };
    let Ok(g1) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
    let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
    let terms = vec![build_pairing_product_term(PairingProductTermOverrides {
        g1: Some(g1.point),
        g2: Some(g2.point),
    })];

    // Act
    let Ok(product) = pairing.pairing_product(
        PairingProductParams,
        build_pairing_product_payload(PairingProductPayloadOverrides { terms: Some(terms) }),
    );
    let Ok(encoded) = pairing.encode_gt(
        EncodeGtParams,
        build_encode_gt_payload(EncodeGtPayloadOverrides {
            value: Some(product.product),
        }),
    );

    // Assert
    assert_eq!(encoded.bytes.expose(), &expected);
}

/// Contract: `reduced_pairing_correction` is the inverse in the scalar field
///   of three, the multiple the library's final exponentiation applies.
/// Arrange: the pairing adapter.
/// Act:     read `pairing.reduced_pairing_correction`.
/// Assert:  `Fr::from(3u64) * reduced_pairing_correction` equals `Fr::ONE`.
#[test]
fn reduced_pairing_correction_inverts_three() {
    // Arrange
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);

    // Act
    let correction = pairing.reduced_pairing_correction;

    // Assert
    assert_eq!(Fr::from(3u64) * correction, Fr::ONE);
}
