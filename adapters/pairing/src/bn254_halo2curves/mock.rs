#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    Bn254Halo2curvesEncodedGt, Bn254Halo2curvesEncodedScalar, Bn254Halo2curvesG1,
    Bn254Halo2curvesG2, Bn254Halo2curvesGt, Bn254Halo2curvesPairing,
    Bn254Halo2curvesPairingConstructorParams, Bn254Halo2curvesScalar,
};
#[cfg(test)]
use core::iter::successors;
#[cfg(test)]
use halo2curves::CurveAffine;
use halo2curves::bn256::{Bn256, Fr, G1Affine, G2Affine, Gt};
#[cfg(test)]
use halo2curves::bn256::{Fq, Fq2, Fq12};
use halo2curves::ff::Field;
#[cfg(test)]
use halo2curves::ff::PrimeField;
#[cfg(test)]
use halo2curves::group::Curve;
#[cfg(test)]
use halo2curves::group::cofactor::CofactorGroup;
#[cfg(test)]
use halo2curves::group::prime::PrimeCurveAffine;
use halo2curves::pairing::Engine;
#[cfg(test)]
use halo2curves::pairing::MultiMillerLoop;
#[cfg(test)]
use hex::decode;
#[cfg(test)]
use num_bigint::BigUint;
#[cfg(test)]
use zeroize::ZeroizeOnDrop;

impl Default for Bn254Halo2curvesScalar {
    fn default() -> Self {
        Bn254Halo2curvesScalar { value: Fr::ONE }
    }
}

impl Default for Bn254Halo2curvesG1 {
    fn default() -> Self {
        Bn254Halo2curvesG1 {
            value: G1Affine::generator(),
        }
    }
}

impl Default for Bn254Halo2curvesG2 {
    fn default() -> Self {
        Bn254Halo2curvesG2 {
            value: G2Affine::generator(),
        }
    }
}

impl Default for Bn254Halo2curvesGt {
    fn default() -> Self {
        Bn254Halo2curvesGt {
            value: Bn256::pairing(&G1Affine::generator(), &G2Affine::generator()),
        }
    }
}

#[derive(Default)]
pub(crate) struct Bn254Halo2curvesScalarOverrides {
    pub value: Option<Fr>,
}

pub(crate) fn build_bn254_halo2curves_scalar(
    overrides: Bn254Halo2curvesScalarOverrides,
) -> Bn254Halo2curvesScalar {
    Bn254Halo2curvesScalar {
        value: overrides
            .value
            .unwrap_or_else(|| Bn254Halo2curvesScalar::default().value),
    }
}

#[derive(Default)]
pub(crate) struct Bn254Halo2curvesG1Overrides {
    pub value: Option<G1Affine>,
}

pub(crate) fn build_bn254_halo2curves_g1(
    overrides: Bn254Halo2curvesG1Overrides,
) -> Bn254Halo2curvesG1 {
    Bn254Halo2curvesG1 {
        value: overrides
            .value
            .unwrap_or_else(|| Bn254Halo2curvesG1::default().value),
    }
}

#[derive(Default)]
pub(crate) struct Bn254Halo2curvesG2Overrides {
    pub value: Option<G2Affine>,
}

pub(crate) fn build_bn254_halo2curves_g2(
    overrides: Bn254Halo2curvesG2Overrides,
) -> Bn254Halo2curvesG2 {
    Bn254Halo2curvesG2 {
        value: overrides
            .value
            .unwrap_or_else(|| Bn254Halo2curvesG2::default().value),
    }
}

#[derive(Default)]
pub(crate) struct Bn254Halo2curvesGtOverrides {
    pub value: Option<Gt>,
}

pub(crate) fn build_bn254_halo2curves_gt(
    overrides: Bn254Halo2curvesGtOverrides,
) -> Bn254Halo2curvesGt {
    Bn254Halo2curvesGt {
        value: overrides
            .value
            .unwrap_or_else(|| Bn254Halo2curvesGt::default().value),
    }
}

pub(crate) fn build_bn254_halo2curves_pairing() -> Bn254Halo2curvesPairing {
    let Ok(pairing) = Bn254Halo2curvesPairing::try_new(Bn254Halo2curvesPairingConstructorParams);
    pairing
}

#[derive(Default)]
pub(crate) struct Bn254Halo2curvesEncodedScalarOverrides {
    pub bytes: Option<[u8; 32]>,
}

pub(crate) fn build_bn254_halo2curves_encoded_scalar(
    overrides: Bn254Halo2curvesEncodedScalarOverrides,
) -> Bn254Halo2curvesEncodedScalar {
    Bn254Halo2curvesEncodedScalar {
        bytes: overrides.bytes.unwrap_or([1u8; 32]),
    }
}

#[derive(Default)]
pub(crate) struct Bn254Halo2curvesEncodedGtOverrides {
    pub bytes: Option<[u8; 384]>,
}

pub(crate) fn build_bn254_halo2curves_encoded_gt(
    overrides: Bn254Halo2curvesEncodedGtOverrides,
) -> Bn254Halo2curvesEncodedGt {
    Bn254Halo2curvesEncodedGt {
        bytes: overrides.bytes.unwrap_or([1u8; 384]),
    }
}

#[cfg(test)]
pub(crate) const BASE_FIELD_MODULUS_HEX: &str =
    "30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47";

#[cfg(test)]
pub(crate) const GROUP_ORDER_HEX: &str =
    "30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001";

#[cfg(test)]
pub(crate) const GROUP_ORDER_MINUS_ONE_HEX: &str =
    "30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000000";

#[cfg(test)]
pub(crate) const GROUP_ORDER_MINUS_TWO_HEX: &str =
    "30644e72e131a029b85045b68181585d2833e84879b9709143e1f593efffffff";

#[cfg(test)]
pub(crate) const G1_GENERATOR_HEX: &str = concat!(
    "0000000000000000000000000000000000000000000000000000000000000001",
    "0000000000000000000000000000000000000000000000000000000000000002"
);

#[cfg(test)]
pub(crate) const G2_GENERATOR_HEX: &str = concat!(
    "198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c2",
    "1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed",
    "090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b",
    "12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa"
);

#[cfg(test)]
pub(crate) const G2_GENERATOR_OFF_CURVE_HEX: &str = concat!(
    "198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c2",
    "1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed",
    "090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b",
    "12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7dab"
);

#[cfg(test)]
pub(crate) const G2_X_C1_AT_MODULUS_HEX: &str = concat!(
    "30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47",
    "1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed",
    "090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b",
    "12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa"
);

#[cfg(test)]
pub(crate) const NEG_G1_GENERATOR_HEX: &str = concat!(
    "0000000000000000000000000000000000000000000000000000000000000001",
    "30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd45"
);

#[cfg(test)]
pub(crate) const G1_X_AT_MODULUS_HEX: &str = concat!(
    "30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47",
    "0000000000000000000000000000000000000000000000000000000000000002"
);

#[cfg(test)]
pub(crate) const G1_OFF_CURVE_HEX: &str = concat!(
    "0000000000000000000000000000000000000000000000000000000000000001",
    "0000000000000000000000000000000000000000000000000000000000000003"
);

#[cfg(test)]
pub(crate) const UNIFORM_GROUP_ORDER_HEX: &str = concat!(
    "0000000000000000000000000000000000000000000000000000000000000000",
    "30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001"
);

#[cfg(test)]
pub(crate) const UNIFORM_FIVE_HEX: &str = concat!(
    "0000000000000000000000000000000000000000000000000000000000000000",
    "0000000000000000000000000000000000000000000000000000000000000005"
);

#[cfg(test)]
pub(crate) fn vector_bytes(hex: &str) -> Vec<u8> {
    let Ok(bytes) = decode(hex) else {
        panic!("the vector decodes")
    };
    bytes
}

#[cfg(test)]
pub(crate) fn zero_bytes(length: usize) -> Vec<u8> {
    vec![0u8; length]
}

#[cfg(test)]
pub(crate) fn gt_identity_encoding() -> Vec<u8> {
    let mut bytes = vec![0u8; 384];
    bytes[31] = 1;
    bytes
}

#[cfg(test)]
pub(crate) fn base_field_modulus() -> BigUint {
    BigUint::from_bytes_be(&vector_bytes(BASE_FIELD_MODULUS_HEX))
}

#[cfg(test)]
pub(crate) fn requires_zeroize_on_drop<T: ZeroizeOnDrop>() {}

#[cfg(test)]
pub(crate) fn scalar_value(hex: &str) -> Fr {
    let mut repr = [0u8; 32];
    repr.copy_from_slice(&vector_bytes(hex));
    repr.reverse();
    let Some(scalar) = Option::<Fr>::from(Fr::from_repr(repr.into())) else {
        panic!("the scalar is canonical")
    };
    scalar
}

#[cfg(test)]
pub(crate) fn eip_196_generator() -> G1Affine {
    let Some(generator) =
        Option::<G1Affine>::from(G1Affine::from_xy(Fq::from(1u64), Fq::from(2u64)))
    else {
        panic!("the generator is on the curve")
    };
    generator
}

#[cfg(test)]
pub(crate) fn eip_196_negated_generator() -> G1Affine {
    let bytes = vector_bytes(NEG_G1_GENERATOR_HEX);
    let mut x_repr = [0u8; 32];
    x_repr.copy_from_slice(&bytes[0..32]);
    x_repr.reverse();
    let mut y_repr = [0u8; 32];
    y_repr.copy_from_slice(&bytes[32..64]);
    y_repr.reverse();
    let Some(x) = Option::<Fq>::from(Fq::from_repr(x_repr.into())) else {
        panic!("the x coordinate is canonical")
    };
    let Some(y) = Option::<Fq>::from(Fq::from_repr(y_repr.into())) else {
        panic!("the y coordinate is canonical")
    };
    let Some(point) = Option::<G1Affine>::from(G1Affine::from_xy(x, y)) else {
        panic!("the point is on the curve")
    };
    point
}

#[cfg(test)]
pub(crate) fn eip_197_generator() -> G2Affine {
    let Some(generator) = g2_point_from_encoding(&vector_bytes(G2_GENERATOR_HEX)) else {
        panic!("the generator is on the curve")
    };
    generator
}

#[cfg(test)]
pub(crate) fn g2_point_from_encoding(bytes: &[u8]) -> Option<G2Affine> {
    let mut x_c1_repr = [0u8; 32];
    x_c1_repr.copy_from_slice(&bytes[0..32]);
    x_c1_repr.reverse();
    let mut x_c0_repr = [0u8; 32];
    x_c0_repr.copy_from_slice(&bytes[32..64]);
    x_c0_repr.reverse();
    let mut y_c1_repr = [0u8; 32];
    y_c1_repr.copy_from_slice(&bytes[64..96]);
    y_c1_repr.reverse();
    let mut y_c0_repr = [0u8; 32];
    y_c0_repr.copy_from_slice(&bytes[96..128]);
    y_c0_repr.reverse();
    let Some(x_c1) = Option::<Fq>::from(Fq::from_repr(x_c1_repr.into())) else {
        panic!("the x c1 coordinate is canonical")
    };
    let Some(x_c0) = Option::<Fq>::from(Fq::from_repr(x_c0_repr.into())) else {
        panic!("the x c0 coordinate is canonical")
    };
    let Some(y_c1) = Option::<Fq>::from(Fq::from_repr(y_c1_repr.into())) else {
        panic!("the y c1 coordinate is canonical")
    };
    let Some(y_c0) = Option::<Fq>::from(Fq::from_repr(y_c0_repr.into())) else {
        panic!("the y c0 coordinate is canonical")
    };
    Option::<G2Affine>::from(G2Affine::from_xy(
        Fq2::new(x_c0, x_c1),
        Fq2::new(y_c0, y_c1),
    ))
}

#[cfg(test)]
pub(crate) fn g2_outside_subgroup_bytes() -> Vec<u8> {
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
    let mut bytes = Vec::with_capacity(128);
    bytes.extend(x.c1().to_repr().as_ref().iter().rev());
    bytes.extend(x.c0().to_repr().as_ref().iter().rev());
    bytes.extend(y.c1().to_repr().as_ref().iter().rev());
    bytes.extend(y.c0().to_repr().as_ref().iter().rev());
    bytes
}

#[cfg(test)]
pub(crate) fn definition_exponent() -> Vec<u64> {
    let Some(p) = BigUint::parse_bytes(Fq::MODULUS.trim_start_matches("0x").as_bytes(), 16) else {
        panic!("the base field modulus parses")
    };
    let Some(r) = BigUint::parse_bytes(Fr::MODULUS.trim_start_matches("0x").as_bytes(), 16) else {
        panic!("the group order parses")
    };
    ((p.pow(12) - BigUint::from(1u32)) / r).to_u64_digits()
}

#[cfg(test)]
pub(crate) fn definition_value(g1: G1Affine, g2: G2Affine) -> Fq12 {
    Bn256::multi_miller_loop(&[(&g1, &g2)]).pow_vartime(definition_exponent())
}

#[cfg(test)]
pub(crate) fn definition_value_power(g1: G1Affine, g2: G2Affine, exponent: u64) -> Fq12 {
    definition_value(g1, g2).pow_vartime([exponent])
}

#[cfg(test)]
pub(crate) fn definition_encoding() -> Vec<u8> {
    let value = definition_value(eip_196_generator(), eip_197_generator());
    let coefficients = [
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
    ];
    let mut bytes = Vec::with_capacity(384);
    for coefficient in coefficients {
        bytes.extend(coefficient.to_repr().as_ref().iter().rev());
    }
    bytes
}

#[cfg(test)]
pub(crate) fn eip_196_doubled_generator() -> G1Affine {
    (eip_196_generator().to_curve() + eip_196_generator().to_curve()).to_affine()
}
