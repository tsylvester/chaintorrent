#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    Bn254Halo2curvesPairing, Bn254Halo2curvesPairingConstructorParams, Bn254Halo2curvesScalar,
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
use core::iter::successors;
use domain::{SecretConstructorParamsOverrides, build_secret};
use halo2curves::CurveAffine;
use halo2curves::bn256::{Fq, Fq2, G2Affine};
use halo2curves::ff::{Field, PrimeField};
use halo2curves::group::{cofactor::CofactorGroup, prime::PrimeCurveAffine};
use hex::decode;
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParamsOverrides, CreateRandomSourcePayload,
    FillBytesParams, FillBytesPayloadOverrides, RandomSourceKind,
    build_create_random_source_params, build_fill_bytes_payload, create_random_source,
};

const BASE_FIELD_MODULUS_HEX: &str =
    "30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47";
const GROUP_ORDER_HEX: &str = "30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001";
const GROUP_ORDER_MINUS_ONE_HEX: &str =
    "30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000000";
const GROUP_ORDER_MINUS_TWO_HEX: &str =
    "30644e72e131a029b85045b68181585d2833e84879b9709143e1f593efffffff";
const G1_GENERATOR_HEX: &str = "0000000000000000000000000000000000000000000000000000000000000001\
     0000000000000000000000000000000000000000000000000000000000000002";
const G2_GENERATOR_HEX: &str = "198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c2\
     1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed\
     090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b\
     12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa";
const G2_GENERATOR_OFF_CURVE_HEX: &str = "198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c2\
     1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed\
     090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b\
     12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7dab";
const SCALAR_TWO_HEX: &str = "0000000000000000000000000000000000000000000000000000000000000002";
const SCALAR_THREE_HEX: &str = "0000000000000000000000000000000000000000000000000000000000000003";
const SCALAR_FIVE_HEX: &str = "0000000000000000000000000000000000000000000000000000000000000005";
const COORDINATE_ONE_HEX: &str = "0000000000000000000000000000000000000000000000000000000000000001";
const COORDINATE_THREE_HEX: &str =
    "0000000000000000000000000000000000000000000000000000000000000003";

/// Contract: the generated branch returns the first-group generator, whose
///   precompile encoding is EIP-196's 64 bytes — the 32-byte x then the
///   32-byte y of the generator, big-endian.
/// Arrange: the pairing adapter.
/// Act:     `pairing.g1_generator`, then `pairing.encode_g1` over it.
/// Assert:  the encoded bytes equal the EIP-196 generator vector.
#[test]
fn g1_generator_encodes_to_the_eip_196_generator() {
    // Arrange
    let Ok(pairing) = Bn254Halo2curvesPairing::try_new(Bn254Halo2curvesPairingConstructorParams);
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
///   precompile encoding is EIP-197's 128 bytes — each coordinate's imaginary
///   part first.
/// Arrange: the pairing adapter.
/// Act:     `pairing.g2_generator`, then `pairing.encode_g2` over it.
/// Assert:  the encoded bytes equal the EIP-197 generator vector.
#[test]
fn g2_generator_encodes_to_the_eip_197_generator() {
    // Arrange
    let Ok(pairing) = Bn254Halo2curvesPairing::try_new(Bn254Halo2curvesPairingConstructorParams);
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
///   re-encodes to the same EIP-196 bytes.
/// Arrange: the pairing adapter and the EIP-196 generator vector.
/// Act:     `pairing.decode_g1` over the vector, then `pairing.encode_g1`.
/// Assert:  the encoded bytes equal the vector.
#[test]
fn decode_g1_round_trips_the_eip_196_generator() {
    // Arrange
    let Ok(pairing) = Bn254Halo2curvesPairing::try_new(Bn254Halo2curvesPairingConstructorParams);
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
///   re-encodes to the same EIP-197 bytes.
/// Arrange: the pairing adapter and the EIP-197 generator vector.
/// Act:     `pairing.decode_g2` over the vector, then `pairing.encode_g2`.
/// Assert:  the encoded bytes equal the vector.
#[test]
fn decode_g2_round_trips_the_eip_197_generator() {
    // Arrange
    let Ok(pairing) = Bn254Halo2curvesPairing::try_new(Bn254Halo2curvesPairingConstructorParams);
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
/// Arrange: the pairing adapter, 64 zero bytes, and the EIP-196 generator
///   vector.
/// Act:     `pairing.decode_g1` over the zeros, `pairing.add_g1` of the
///   generator and the decoded point, then `pairing.encode_g1`.
/// Assert:  the encoded bytes equal the EIP-196 generator vector.
#[test]
fn decode_g1_reads_the_all_zero_encoding_as_the_identity() {
    // Arrange
    let Ok(pairing) = Bn254Halo2curvesPairing::try_new(Bn254Halo2curvesPairingConstructorParams);
    let Ok(expected) = decode(G1_GENERATOR_HEX) else {
        panic!("the vector decodes")
    };
    let zeros = vec![0u8; 64];
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
/// Arrange: the pairing adapter, 128 zero bytes, and the EIP-197 generator
///   vector.
/// Act:     `pairing.decode_g2` over the zeros, `pairing.add_g2` of the
///   generator and the decoded point, then `pairing.encode_g2`.
/// Assert:  the encoded bytes equal the EIP-197 generator vector.
#[test]
fn decode_g2_reads_the_all_zero_encoding_as_the_identity() {
    // Arrange
    let Ok(pairing) = Bn254Halo2curvesPairing::try_new(Bn254Halo2curvesPairingConstructorParams);
    let Ok(expected) = decode(G2_GENERATOR_HEX) else {
        panic!("the vector decodes")
    };
    let zeros = vec![0u8; 128];
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
///   is none — writes the identity as 64 zero bytes.
/// Arrange: the pairing adapter and the identity produced by `msm_g1` over an
///   empty term list.
/// Act:     `pairing.msm_g1` over the empty payload, then `pairing.encode_g1`.
/// Assert:  the encoded bytes equal 64 zero bytes.
#[test]
fn encode_g1_writes_the_identity_as_all_zero_bytes() {
    // Arrange
    let Ok(pairing) = Bn254Halo2curvesPairing::try_new(Bn254Halo2curvesPairingConstructorParams);
    let expected = vec![0u8; 64];

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
///   `WrongLength { expected: 64, actual }`.
/// Arrange: the pairing adapter and 63 bytes.
/// Act:     `pairing.decode_g1` over the bytes.
/// Assert:  the return is `Err(DecodeG1ErrorReturn::WrongLength { expected:
///   64, actual: 63 })`.
#[test]
fn decode_g1_rejects_a_wrong_length() {
    // Arrange
    let Ok(pairing) = Bn254Halo2curvesPairing::try_new(Bn254Halo2curvesPairingConstructorParams);
    let bytes = vec![0u8; 63];

    // Act
    let result = pairing.decode_g1(DecodeG1Params, &bytes);

    // Assert
    assert!(matches!(
        result,
        Err(DecodeG1ErrorReturn::WrongLength {
            expected: 64,
            actual: 63
        })
    ));
}

/// Contract: the non-canonical-coordinate branch of `decode_g1` — a 32-byte
///   half at least the base field modulus — returns `NonCanonicalCoordinate`.
/// Arrange: the pairing adapter, the base field modulus, and the 32-byte
///   encoding of two.
/// Act:     `pairing.decode_g1` over the concatenated bytes.
/// Assert:  the return is `Err(DecodeG1ErrorReturn::NonCanonicalCoordinate)`.
#[test]
fn decode_g1_rejects_a_coordinate_equal_to_the_base_field_modulus() {
    // Arrange
    let Ok(pairing) = Bn254Halo2curvesPairing::try_new(Bn254Halo2curvesPairingConstructorParams);
    let Ok(mut bytes) = decode(BASE_FIELD_MODULUS_HEX) else {
        panic!("the modulus decodes")
    };
    let Ok(two) = decode(SCALAR_TWO_HEX) else {
        panic!("the coordinate decodes")
    };
    bytes.extend_from_slice(&two);

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
/// Arrange: the pairing adapter and the 32-byte encodings of one and three;
///   (1, 3) satisfies y^2 = x^3 + 3 on neither side, 9 against 4.
/// Act:     `pairing.decode_g1` over the concatenated bytes.
/// Assert:  the return is `Err(DecodeG1ErrorReturn::NotOnCurve)`.
#[test]
fn decode_g1_rejects_a_point_off_the_curve() {
    // Arrange
    let Ok(pairing) = Bn254Halo2curvesPairing::try_new(Bn254Halo2curvesPairingConstructorParams);
    let Ok(mut bytes) = decode(COORDINATE_ONE_HEX) else {
        panic!("the coordinate decodes")
    };
    let Ok(three) = decode(COORDINATE_THREE_HEX) else {
        panic!("the coordinate decodes")
    };
    bytes.extend_from_slice(&three);

    // Act
    let result = pairing.decode_g1(DecodeG1Params, &bytes);

    // Assert
    assert!(matches!(result, Err(DecodeG1ErrorReturn::NotOnCurve)));
}

/// Contract: the wrong-length branch of `decode_g2` returns
///   `WrongLength { expected: 128, actual }`.
/// Arrange: the pairing adapter and 127 bytes.
/// Act:     `pairing.decode_g2` over the bytes.
/// Assert:  the return is `Err(DecodeG2ErrorReturn::WrongLength { expected:
///   128, actual: 127 })`.
#[test]
fn decode_g2_rejects_a_wrong_length() {
    // Arrange
    let Ok(pairing) = Bn254Halo2curvesPairing::try_new(Bn254Halo2curvesPairingConstructorParams);
    let bytes = vec![0u8; 127];

    // Act
    let result = pairing.decode_g2(DecodeG2Params, &bytes);

    // Assert
    assert!(matches!(
        result,
        Err(DecodeG2ErrorReturn::WrongLength {
            expected: 128,
            actual: 127
        })
    ));
}

/// Contract: the off-the-curve branch of `decode_g2` returns `NotOnCurve`.
/// Arrange: the pairing adapter and the EIP-197 generator vector with its
///   last byte `aa` replaced by `ab`, a canonical pair off the curve.
/// Act:     `pairing.decode_g2` over the bytes.
/// Assert:  the return is `Err(DecodeG2ErrorReturn::NotOnCurve)`.
#[test]
fn decode_g2_rejects_a_point_off_the_curve() {
    // Arrange
    let Ok(pairing) = Bn254Halo2curvesPairing::try_new(Bn254Halo2curvesPairingConstructorParams);
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
///   from the square root of `x^3 + b`, encoded as `x.c1`, `x.c0`, `y.c1`,
///   `y.c0`, each 32 bytes big-endian.
/// Act:     `pairing.decode_g2` over the encoded point.
/// Assert:  the return is `Err(DecodeG2ErrorReturn::NotInSubgroup)`.
#[test]
fn decode_g2_rejects_a_point_outside_the_subgroup() {
    // Arrange
    let Ok(pairing) = Bn254Halo2curvesPairing::try_new(Bn254Halo2curvesPairingConstructorParams);
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
    bytes.extend(x.c1().to_repr().as_ref().iter().rev());
    bytes.extend(x.c0().to_repr().as_ref().iter().rev());
    bytes.extend(y.c1().to_repr().as_ref().iter().rev());
    bytes.extend(y.c0().to_repr().as_ref().iter().rev());

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
    let Ok(pairing) = Bn254Halo2curvesPairing::try_new(Bn254Halo2curvesPairingConstructorParams);
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
    let Ok(pairing) = Bn254Halo2curvesPairing::try_new(Bn254Halo2curvesPairingConstructorParams);
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
    let Ok(pairing) = Bn254Halo2curvesPairing::try_new(Bn254Halo2curvesPairingConstructorParams);
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
    let Ok(pairing) = Bn254Halo2curvesPairing::try_new(Bn254Halo2curvesPairingConstructorParams);
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
    let Ok(pairing) = Bn254Halo2curvesPairing::try_new(Bn254Halo2curvesPairingConstructorParams);
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
    let Ok(pairing) = Bn254Halo2curvesPairing::try_new(Bn254Halo2curvesPairingConstructorParams);
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
    let Ok(pairing) = Bn254Halo2curvesPairing::try_new(Bn254Halo2curvesPairingConstructorParams);
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
    let Ok(pairing) = Bn254Halo2curvesPairing::try_new(Bn254Halo2curvesPairingConstructorParams);
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
    let Ok(pairing) = Bn254Halo2curvesPairing::try_new(Bn254Halo2curvesPairingConstructorParams);
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
    let Ok(pairing) = Bn254Halo2curvesPairing::try_new(Bn254Halo2curvesPairingConstructorParams);
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
/// Act:     `Bn254Halo2curvesScalar::sample_from_uniform_bytes` over the
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
        Bn254Halo2curvesScalar::sample_from_uniform_bytes(SampleUniformScalarParams, payload);

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
/// Act:     `Bn254Halo2curvesScalar::sample_from_uniform_bytes` over the
///   payload, then `encode_scalar` of a clone of the exposed scalar.
/// Assert:  the exposed encoded bytes equal the 32-byte big-endian encoding
///   of five.
#[test]
fn sample_from_uniform_bytes_reads_its_input_as_a_big_endian_integer() {
    // Arrange
    let Ok(pairing) = Bn254Halo2curvesPairing::try_new(Bn254Halo2curvesPairingConstructorParams);
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
        Bn254Halo2curvesScalar::sample_from_uniform_bytes(SampleUniformScalarParams, payload)
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
/// Act:     `Bn254Halo2curvesScalar::sample_from_uniform_bytes` over the
///   payload, then `encode_scalar` of a clone of the exposed scalar, then
///   `decode_scalar` over the encoding.
/// Assert:  the decode returns `Ok` — the sampled scalar is below `r`.
#[test]
fn sample_from_uniform_bytes_reduces_the_largest_input_into_the_scalar_field() {
    // Arrange
    let Ok(pairing) = Bn254Halo2curvesPairing::try_new(Bn254Halo2curvesPairingConstructorParams);
    let payload = build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
        uniform: Some(build_secret(SecretConstructorParamsOverrides {
            value: Some(vec![0xffu8; 64]),
        })),
    });

    // Act
    let Ok(sampled) =
        Bn254Halo2curvesScalar::sample_from_uniform_bytes(SampleUniformScalarParams, payload)
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
///   `Bn254Halo2curvesScalar::sample_from_uniform_bytes` over the draw, then
///   `encode_scalar` and `decode_scalar`.
/// Assert:  the decode returns `Ok` — the sampled scalar is below `r`.
#[test]
fn sample_from_uniform_bytes_draws_an_in_range_scalar_from_the_operating_system_source() {
    // Arrange
    let Ok(pairing) = Bn254Halo2curvesPairing::try_new(Bn254Halo2curvesPairingConstructorParams);
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
            length: Some(Bn254Halo2curvesScalar::UNIFORM_BYTES_LENGTH),
        }),
    ) else {
        panic!("the source draws")
    };
    let payload = build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
        uniform: Some(drawn.bytes),
    });

    // Act
    let Ok(sampled) =
        Bn254Halo2curvesScalar::sample_from_uniform_bytes(SampleUniformScalarParams, payload)
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
/// Act:     read `Bn254Halo2curvesPairing::DECLARATION`.
/// Assert:  `curve` is `PairingCurve::Bn254`, `verifier_group_arithmetic` is
///   `VerifierGroupArithmetic::FirstGroupOnly`, `precompile_encoding` is
///   `PrecompileEncoding::Eip196Eip197`, `adapter_version` equals 1, and
///   `interface_version` equals `PAIRING_INTERFACE_VERSION`.
#[test]
fn bn254_halo2curves_pairing_declares_its_curve_arithmetic_encoding_and_versions() {
    // Arrange

    // Act
    let declaration = Bn254Halo2curvesPairing::DECLARATION;

    // Assert
    assert!(matches!(declaration.curve, PairingCurve::Bn254));
    assert!(matches!(
        declaration.verifier_group_arithmetic,
        VerifierGroupArithmetic::FirstGroupOnly
    ));
    assert!(matches!(
        declaration.precompile_encoding,
        PrecompileEncoding::Eip196Eip197
    ));
    assert_eq!(declaration.adapter_version, 1);
    assert_eq!(declaration.interface_version, PAIRING_INTERFACE_VERSION);
}
