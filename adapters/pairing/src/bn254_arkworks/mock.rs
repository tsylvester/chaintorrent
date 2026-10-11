#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

#[cfg(test)]
use super::BN254_SEED;
use super::interface::{
    Bn254ArkworksEncodedGt, Bn254ArkworksEncodedScalar, Bn254ArkworksG1, Bn254ArkworksG2,
    Bn254ArkworksGt, Bn254ArkworksPairing, Bn254ArkworksPairingConstructorParams,
    Bn254ArkworksScalar,
};
use ark_bn254::{Bn254, Fr, G1Affine, G2Affine};
#[cfg(test)]
use ark_bn254::{Fq, Fq2, Fq6, Fq12};
#[cfg(test)]
use ark_ec::CurveGroup;
use ark_ec::{
    AffineRepr,
    pairing::{Pairing, PairingOutput},
};
#[cfg(test)]
use ark_ff::{BigInteger, Field, PrimeField};
#[cfg(test)]
use hex::decode;
#[cfg(test)]
use num_bigint::BigUint;
#[cfg(test)]
use zeroize::ZeroizeOnDrop;

#[derive(Default)]
pub(crate) struct Bn254ArkworksScalarOverrides {
    pub value: Option<Fr>,
}

pub(crate) fn build_bn254_arkworks_scalar(
    overrides: Bn254ArkworksScalarOverrides,
) -> Bn254ArkworksScalar {
    Bn254ArkworksScalar {
        value: overrides.value.unwrap_or_else(|| Fr::from(1u64)),
    }
}

#[derive(Default)]
pub(crate) struct Bn254ArkworksG1Overrides {
    pub value: Option<G1Affine>,
}

pub(crate) fn build_bn254_arkworks_g1(overrides: Bn254ArkworksG1Overrides) -> Bn254ArkworksG1 {
    Bn254ArkworksG1 {
        value: overrides.value.unwrap_or_else(G1Affine::generator),
    }
}

#[derive(Default)]
pub(crate) struct Bn254ArkworksG2Overrides {
    pub value: Option<G2Affine>,
}

pub(crate) fn build_bn254_arkworks_g2(overrides: Bn254ArkworksG2Overrides) -> Bn254ArkworksG2 {
    Bn254ArkworksG2 {
        value: overrides.value.unwrap_or_else(G2Affine::generator),
    }
}

#[derive(Default)]
pub(crate) struct Bn254ArkworksGtOverrides {
    pub value: Option<PairingOutput<Bn254>>,
}

pub(crate) fn build_bn254_arkworks_gt(overrides: Bn254ArkworksGtOverrides) -> Bn254ArkworksGt {
    Bn254ArkworksGt {
        value: overrides
            .value
            .unwrap_or_else(|| Bn254::pairing(G1Affine::generator(), G2Affine::generator())),
    }
}

pub(crate) fn build_bn254_arkworks_pairing() -> Bn254ArkworksPairing {
    let Ok(pairing) = Bn254ArkworksPairing::try_new(Bn254ArkworksPairingConstructorParams);
    pairing
}

#[derive(Default)]
pub(crate) struct Bn254ArkworksEncodedScalarOverrides {
    pub bytes: Option<[u8; 32]>,
}

pub(crate) fn build_bn254_arkworks_encoded_scalar(
    overrides: Bn254ArkworksEncodedScalarOverrides,
) -> Bn254ArkworksEncodedScalar {
    Bn254ArkworksEncodedScalar {
        bytes: overrides.bytes.unwrap_or([1u8; 32]),
    }
}

#[derive(Default)]
pub(crate) struct Bn254ArkworksEncodedGtOverrides {
    pub bytes: Option<[u8; 384]>,
}

pub(crate) fn build_bn254_arkworks_encoded_gt(
    overrides: Bn254ArkworksEncodedGtOverrides,
) -> Bn254ArkworksEncodedGt {
    Bn254ArkworksEncodedGt {
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
pub(crate) fn gt_with_sequential_coefficients() -> PairingOutput<Bn254> {
    PairingOutput(Fq12::new(
        Fq6::new(
            Fq2::new(Fq::from(1u64), Fq::from(2u64)),
            Fq2::new(Fq::from(3u64), Fq::from(4u64)),
            Fq2::new(Fq::from(5u64), Fq::from(6u64)),
        ),
        Fq6::new(
            Fq2::new(Fq::from(7u64), Fq::from(8u64)),
            Fq2::new(Fq::from(9u64), Fq::from(10u64)),
            Fq2::new(Fq::from(11u64), Fq::from(12u64)),
        ),
    ))
}

#[cfg(test)]
pub(crate) fn sequential_gt_encoding() -> Vec<u8> {
    let mut bytes = vec![0u8; 384];
    for i in 0..12 {
        bytes[32 * i + 31] = (i + 1) as u8;
    }
    bytes
}

#[cfg(test)]
pub(crate) fn base_field_modulus() -> BigUint {
    BigUint::from_bytes_be(&vector_bytes(BASE_FIELD_MODULUS_HEX))
}

#[cfg(test)]
pub(crate) fn scalar_value(hex: &str) -> Fr {
    Fr::from_be_bytes_mod_order(&vector_bytes(hex))
}

#[cfg(test)]
pub(crate) fn eip_196_generator() -> G1Affine {
    G1Affine::new_unchecked(Fq::from(1u64), Fq::from(2u64))
}

#[cfg(test)]
pub(crate) fn eip_196_negated_generator() -> G1Affine {
    let bytes = vector_bytes(NEG_G1_GENERATOR_HEX);
    G1Affine::new_unchecked(
        Fq::from_be_bytes_mod_order(&bytes[0..32]),
        Fq::from_be_bytes_mod_order(&bytes[32..64]),
    )
}

#[cfg(test)]
pub(crate) fn eip_197_generator() -> G2Affine {
    g2_point_from_encoding(&vector_bytes(G2_GENERATOR_HEX))
}

#[cfg(test)]
pub(crate) fn g2_point_from_encoding(bytes: &[u8]) -> G2Affine {
    let x_c1 = Fq::from_be_bytes_mod_order(&bytes[0..32]);
    let x_c0 = Fq::from_be_bytes_mod_order(&bytes[32..64]);
    let y_c1 = Fq::from_be_bytes_mod_order(&bytes[64..96]);
    let y_c0 = Fq::from_be_bytes_mod_order(&bytes[96..128]);
    G2Affine::new_unchecked(Fq2::new(x_c0, x_c1), Fq2::new(y_c0, y_c1))
}

#[cfg(test)]
pub(crate) fn g2_outside_subgroup_bytes() -> Vec<u8> {
    let Some(point) = (1u64..).find_map(|c0| {
        G2Affine::get_point_from_x_unchecked(Fq2::new(Fq::from(c0), Fq::from(0u64)), false)
            .filter(|point| !point.is_in_correct_subgroup_assuming_on_curve())
    }) else {
        panic!("an on-curve point outside the subgroup exists")
    };
    let Some((x, y)) = point.xy() else {
        panic!("the point has coordinates")
    };
    let mut bytes = Vec::with_capacity(128);
    bytes.extend_from_slice(&x.c1.into_bigint().to_bytes_be());
    bytes.extend_from_slice(&x.c0.into_bigint().to_bytes_be());
    bytes.extend_from_slice(&y.c1.into_bigint().to_bytes_be());
    bytes.extend_from_slice(&y.c0.into_bigint().to_bytes_be());
    bytes
}

#[cfg(test)]
pub(crate) fn definition_exponent() -> Vec<u64> {
    let p = BigUint::from(Fq::MODULUS);
    let r = BigUint::from(Fr::MODULUS);
    ((p.pow(12u32) - BigUint::from(1u32)) / r).to_u64_digits()
}

#[cfg(test)]
pub(crate) fn definition_value(g1: G1Affine, g2: G2Affine) -> Fq12 {
    Bn254::multi_miller_loop([g1], [g2])
        .0
        .pow(definition_exponent())
}

#[cfg(test)]
pub(crate) fn eip_196_doubled_generator() -> G1Affine {
    (eip_196_generator() + eip_196_generator()).into_affine()
}

#[cfg(test)]
pub(crate) fn definition_value_power(g1: G1Affine, g2: G2Affine, exponent: u64) -> Fq12 {
    definition_value(g1, g2).pow([exponent])
}

#[cfg(test)]
pub(crate) fn library_multiple() -> Fr {
    let seed = Fr::from(BN254_SEED);
    (seed * seed * Fr::from(6u64) + seed * Fr::from(3u64) + Fr::ONE) * seed * Fr::from(2u64)
}

#[cfg(test)]
pub(crate) fn requires_zeroize_on_drop<T: ZeroizeOnDrop>() {}
