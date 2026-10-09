#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    Bls12381Halo2curvesEncodedGt, Bls12381Halo2curvesEncodedScalar, Bls12381Halo2curvesG1,
    Bls12381Halo2curvesG2, Bls12381Halo2curvesGt, Bls12381Halo2curvesPairing,
    Bls12381Halo2curvesPairingConstructorParams, Bls12381Halo2curvesScalar,
};
#[cfg(test)]
use core::iter::successors;
#[cfg(test)]
use halo2curves::CurveAffine;
use halo2curves::bls12381::{Bls12381, Fr, G1Affine, G2Affine, Gt};
#[cfg(test)]
use halo2curves::bls12381::{Fq, Fq2, Fq12};
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

impl Default for Bls12381Halo2curvesScalar {
    fn default() -> Self {
        Bls12381Halo2curvesScalar { value: Fr::ONE }
    }
}

impl Default for Bls12381Halo2curvesG1 {
    fn default() -> Self {
        Bls12381Halo2curvesG1 {
            value: G1Affine::generator(),
        }
    }
}

impl Default for Bls12381Halo2curvesG2 {
    fn default() -> Self {
        Bls12381Halo2curvesG2 {
            value: G2Affine::generator(),
        }
    }
}

impl Default for Bls12381Halo2curvesGt {
    fn default() -> Self {
        Bls12381Halo2curvesGt {
            value: Bls12381::pairing(&G1Affine::generator(), &G2Affine::generator()),
        }
    }
}

#[derive(Default)]
pub(crate) struct Bls12381Halo2curvesScalarOverrides {
    pub value: Option<Fr>,
}

pub(crate) fn build_bls12_381_halo2curves_scalar(
    overrides: Bls12381Halo2curvesScalarOverrides,
) -> Bls12381Halo2curvesScalar {
    Bls12381Halo2curvesScalar {
        value: overrides
            .value
            .unwrap_or_else(|| Bls12381Halo2curvesScalar::default().value),
    }
}

#[derive(Default)]
pub(crate) struct Bls12381Halo2curvesG1Overrides {
    pub value: Option<G1Affine>,
}

pub(crate) fn build_bls12_381_halo2curves_g1(
    overrides: Bls12381Halo2curvesG1Overrides,
) -> Bls12381Halo2curvesG1 {
    Bls12381Halo2curvesG1 {
        value: overrides
            .value
            .unwrap_or_else(|| Bls12381Halo2curvesG1::default().value),
    }
}

#[derive(Default)]
pub(crate) struct Bls12381Halo2curvesG2Overrides {
    pub value: Option<G2Affine>,
}

pub(crate) fn build_bls12_381_halo2curves_g2(
    overrides: Bls12381Halo2curvesG2Overrides,
) -> Bls12381Halo2curvesG2 {
    Bls12381Halo2curvesG2 {
        value: overrides
            .value
            .unwrap_or_else(|| Bls12381Halo2curvesG2::default().value),
    }
}

#[derive(Default)]
pub(crate) struct Bls12381Halo2curvesGtOverrides {
    pub value: Option<Gt>,
}

pub(crate) fn build_bls12_381_halo2curves_gt(
    overrides: Bls12381Halo2curvesGtOverrides,
) -> Bls12381Halo2curvesGt {
    Bls12381Halo2curvesGt {
        value: overrides
            .value
            .unwrap_or_else(|| Bls12381Halo2curvesGt::default().value),
    }
}

pub(crate) fn build_bls12_381_halo2curves_pairing() -> Bls12381Halo2curvesPairing {
    let Ok(pairing) =
        Bls12381Halo2curvesPairing::try_new(Bls12381Halo2curvesPairingConstructorParams);
    pairing
}

#[derive(Default)]
pub(crate) struct Bls12381Halo2curvesEncodedScalarOverrides {
    pub bytes: Option<[u8; 32]>,
}

pub(crate) fn build_bls12_381_halo2curves_encoded_scalar(
    overrides: Bls12381Halo2curvesEncodedScalarOverrides,
) -> Bls12381Halo2curvesEncodedScalar {
    Bls12381Halo2curvesEncodedScalar {
        bytes: overrides.bytes.unwrap_or([1u8; 32]),
    }
}

#[derive(Default)]
pub(crate) struct Bls12381Halo2curvesEncodedGtOverrides {
    pub bytes: Option<[u8; 576]>,
}

pub(crate) fn build_bls12_381_halo2curves_encoded_gt(
    overrides: Bls12381Halo2curvesEncodedGtOverrides,
) -> Bls12381Halo2curvesEncodedGt {
    Bls12381Halo2curvesEncodedGt {
        bytes: overrides.bytes.unwrap_or([1u8; 576]),
    }
}

#[cfg(test)]
pub(crate) const BASE_FIELD_MODULUS_PADDED_HEX: &str = concat!(
    "00000000000000000000000000000000",
    "1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab"
);

#[cfg(test)]
pub(crate) const GROUP_ORDER_HEX: &str =
    "73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001";

#[cfg(test)]
pub(crate) const GROUP_ORDER_MINUS_ONE_HEX: &str =
    "73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000000";

#[cfg(test)]
pub(crate) const GROUP_ORDER_MINUS_TWO_HEX: &str =
    "73eda753299d7d483339d80809a1d80553bda402fffe5bfefffffffeffffffff";

#[cfg(test)]
pub(crate) const G1_GENERATOR_HEX: &str = concat!(
    "00000000000000000000000000000000",
    "17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb",
    "00000000000000000000000000000000",
    "08b3f481e3aaa0f1a09e30ed741d8ae4fcf5e095d5d00af600db18cb2c04b3edd03cc744a2888ae40caa232946c5e7e1"
);

#[cfg(test)]
pub(crate) const G1_GENERATOR_OFF_CURVE_HEX: &str = concat!(
    "00000000000000000000000000000000",
    "17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb",
    "00000000000000000000000000000000",
    "08b3f481e3aaa0f1a09e30ed741d8ae4fcf5e095d5d00af600db18cb2c04b3edd03cc744a2888ae40caa232946c5e7e2"
);

#[cfg(test)]
pub(crate) const G2_GENERATOR_HEX: &str = concat!(
    "00000000000000000000000000000000",
    "024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8",
    "00000000000000000000000000000000",
    "13e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e",
    "00000000000000000000000000000000",
    "0ce5d527727d6e118cc9cdc6da2e351aadfd9baa8cbdd3a76d429a695160d12c923ac9cc3baca289e193548608b82801",
    "00000000000000000000000000000000",
    "0606c4a02ea734cc32acd2b02bc28b99cb3e287e85a763af267492ab572e99ab3f370d275cec1da1aaa9075ff05f79be"
);

#[cfg(test)]
pub(crate) const G2_GENERATOR_OFF_CURVE_HEX: &str = concat!(
    "00000000000000000000000000000000",
    "024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8",
    "00000000000000000000000000000000",
    "13e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e",
    "00000000000000000000000000000000",
    "0ce5d527727d6e118cc9cdc6da2e351aadfd9baa8cbdd3a76d429a695160d12c923ac9cc3baca289e193548608b82801",
    "00000000000000000000000000000000",
    "0606c4a02ea734cc32acd2b02bc28b99cb3e287e85a763af267492ab572e99ab3f370d275cec1da1aaa9075ff05f79bf"
);

#[cfg(test)]
pub(crate) const NEG_G1_GENERATOR_HEX: &str = concat!(
    "00000000000000000000000000000000",
    "17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb",
    "00000000000000000000000000000000",
    "114d1d6855d545a8aa7d76c8cf2e21f267816aef1db507c96655b9d5caac42364e6f38ba0ecb751bad54dcd6b939c2ca"
);

#[cfg(test)]
pub(crate) const G1_NONZERO_PADDING_HEX: &str = concat!(
    "01000000000000000000000000000000",
    "17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb",
    "00000000000000000000000000000000",
    "08b3f481e3aaa0f1a09e30ed741d8ae4fcf5e095d5d00af600db18cb2c04b3edd03cc744a2888ae40caa232946c5e7e1"
);

#[cfg(test)]
pub(crate) const G1_X_AT_MODULUS_HEX: &str = concat!(
    "00000000000000000000000000000000",
    "1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab",
    "00000000000000000000000000000000",
    "08b3f481e3aaa0f1a09e30ed741d8ae4fcf5e095d5d00af600db18cb2c04b3edd03cc744a2888ae40caa232946c5e7e1"
);

#[cfg(test)]
pub(crate) const G2_NONZERO_PADDING_HEX: &str = concat!(
    "01000000000000000000000000000000",
    "024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8",
    "00000000000000000000000000000000",
    "13e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e",
    "00000000000000000000000000000000",
    "0ce5d527727d6e118cc9cdc6da2e351aadfd9baa8cbdd3a76d429a695160d12c923ac9cc3baca289e193548608b82801",
    "00000000000000000000000000000000",
    "0606c4a02ea734cc32acd2b02bc28b99cb3e287e85a763af267492ab572e99ab3f370d275cec1da1aaa9075ff05f79be"
);

#[cfg(test)]
pub(crate) const G2_X_C0_AT_MODULUS_HEX: &str = concat!(
    "00000000000000000000000000000000",
    "1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab",
    "00000000000000000000000000000000",
    "13e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e",
    "00000000000000000000000000000000",
    "0ce5d527727d6e118cc9cdc6da2e351aadfd9baa8cbdd3a76d429a695160d12c923ac9cc3baca289e193548608b82801",
    "00000000000000000000000000000000",
    "0606c4a02ea734cc32acd2b02bc28b99cb3e287e85a763af267492ab572e99ab3f370d275cec1da1aaa9075ff05f79be"
);

#[cfg(test)]
pub(crate) const UNIFORM_GROUP_ORDER_HEX: &str = concat!(
    "0000000000000000000000000000000000000000000000000000000000000000",
    "73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001"
);

#[cfg(test)]
pub(crate) const UNIFORM_FIVE_HEX: &str = concat!(
    "0000000000000000000000000000000000000000000000000000000000000000",
    "0000000000000000000000000000000000000000000000000000000000000005"
);

#[cfg(test)]
pub(crate) const CFRG_GENERATOR_PAIRING_HEX: &str = concat!(
    "11619b45f61edfe3b47a15fac19442526ff489dcda25e59121d9931438907dfd448299a87dde3a649bdba96e84d54558",
    "153ce14a76a53e205ba8f275ef1137c56a566f638b52d34ba3bf3bf22f277d70f76316218c0dfd583a394b8448d2be7f",
    "095668fb4a02fe930ed44767834c915b283b1c6ca98c047bd4c272e9ac3f3ba6ff0b05a93e59c71fba77bce995f04692",
    "16deedaa683124fe7260085184d88f7d036b86f53bb5b7f1fc5e248814782065413e7d958d17960109ea006b2afdeb5f",
    "09c92cf02f3cd3d2f9d34bc44eee0dd50314ed44ca5d30ce6a9ec0539be7a86b121edc61839ccc908c4bdde256cd6048",
    "111061f398efc2a97ff825b04d21089e24fd8b93a47e41e60eae7e9b2a38d54fa4dedced0811c34ce528781ab9e929c7",
    "01ecfcf31c86257ab00b4709c33f1c9c4e007659dd5ffc4a735192167ce197058cfb4c94225e7f1b6c26ad9ba68f63bc",
    "08890726743a1f94a8193a166800b7787744a8ad8e2f9365db76863e894b7a11d83f90d873567e9d645ccf725b32d26f",
    "0e61c752414ca5dfd258e9606bac08daec29b3e2c57062669556954fb227d3f1260eedf25446a086b0844bcd43646c10",
    "0fe63f185f56dd29150fc498bbeea78969e7e783043620db33f75a05a0a2ce5c442beaff9da195ff15164c00ab66bdde",
    "10900338a92ed0b47af211636f7cfdec717b7ee43900eee9b5fc24f0000c5874d4801372db478987691c566a8c474978",
    "1454814f3085f0e6602247671bc408bbce2007201536818c901dbd4d2095dd86c1ec8b888e59611f60a301af7776be3d"
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
    let mut bytes = vec![0u8; 576];
    bytes[47] = 1;
    bytes
}

#[cfg(test)]
pub(crate) fn base_field_modulus() -> BigUint {
    BigUint::from_bytes_be(&vector_bytes(BASE_FIELD_MODULUS_PADDED_HEX))
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
pub(crate) fn g1_point_from_encoding(bytes: &[u8]) -> Option<G1Affine> {
    let mut x_repr = [0u8; 48];
    x_repr.copy_from_slice(&bytes[16..64]);
    x_repr.reverse();
    let mut y_repr = [0u8; 48];
    y_repr.copy_from_slice(&bytes[80..128]);
    y_repr.reverse();
    let x = Option::<Fq>::from(Fq::from_repr(x_repr.into()))?;
    let y = Option::<Fq>::from(Fq::from_repr(y_repr.into()))?;
    Option::<G1Affine>::from(G1Affine::from_xy(x, y))
}

#[cfg(test)]
pub(crate) fn g2_point_from_encoding(bytes: &[u8]) -> Option<G2Affine> {
    let mut x_c0_repr = [0u8; 48];
    x_c0_repr.copy_from_slice(&bytes[16..64]);
    x_c0_repr.reverse();
    let mut x_c1_repr = [0u8; 48];
    x_c1_repr.copy_from_slice(&bytes[80..128]);
    x_c1_repr.reverse();
    let mut y_c0_repr = [0u8; 48];
    y_c0_repr.copy_from_slice(&bytes[144..192]);
    y_c0_repr.reverse();
    let mut y_c1_repr = [0u8; 48];
    y_c1_repr.copy_from_slice(&bytes[208..256]);
    y_c1_repr.reverse();
    let x_c0 = Option::<Fq>::from(Fq::from_repr(x_c0_repr.into()))?;
    let x_c1 = Option::<Fq>::from(Fq::from_repr(x_c1_repr.into()))?;
    let y_c0 = Option::<Fq>::from(Fq::from_repr(y_c0_repr.into()))?;
    let y_c1 = Option::<Fq>::from(Fq::from_repr(y_c1_repr.into()))?;
    Option::<G2Affine>::from(G2Affine::from_xy(
        Fq2::new(x_c0, x_c1),
        Fq2::new(y_c0, y_c1),
    ))
}

#[cfg(test)]
pub(crate) fn eip_2537_g1_generator() -> G1Affine {
    let Some(point) = g1_point_from_encoding(&vector_bytes(G1_GENERATOR_HEX)) else {
        panic!("the published point is on the curve")
    };
    point
}

#[cfg(test)]
pub(crate) fn eip_2537_negated_g1_generator() -> G1Affine {
    let Some(point) = g1_point_from_encoding(&vector_bytes(NEG_G1_GENERATOR_HEX)) else {
        panic!("the published point is on the curve")
    };
    point
}

#[cfg(test)]
pub(crate) fn eip_2537_g2_generator() -> G2Affine {
    let Some(point) = g2_point_from_encoding(&vector_bytes(G2_GENERATOR_HEX)) else {
        panic!("the published point is on the curve")
    };
    point
}

#[cfg(test)]
pub(crate) fn g1_outside_subgroup_bytes() -> Vec<u8> {
    let Some((x, y)) = successors(Some(Fq::ONE), |x| Some(*x + Fq::ONE)).find_map(|x| {
        Option::<Fq>::from((x.square() * x + G1Affine::b()).sqrt()).and_then(|y| {
            Option::<G1Affine>::from(G1Affine::from_xy(x, y))
                .filter(|point| !bool::from(point.to_curve().is_torsion_free()))
                .map(|_| (x, y))
        })
    }) else {
        panic!("an on-curve point outside the subgroup exists")
    };
    let mut bytes = Vec::with_capacity(128);
    bytes.extend_from_slice(&[0u8; 16]);
    bytes.extend(x.to_repr().as_ref().iter().rev());
    bytes.extend_from_slice(&[0u8; 16]);
    bytes.extend(y.to_repr().as_ref().iter().rev());
    bytes
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
    let mut bytes = Vec::with_capacity(256);
    bytes.extend_from_slice(&[0u8; 16]);
    bytes.extend(x.c0().to_repr().as_ref().iter().rev());
    bytes.extend_from_slice(&[0u8; 16]);
    bytes.extend(x.c1().to_repr().as_ref().iter().rev());
    bytes.extend_from_slice(&[0u8; 16]);
    bytes.extend(y.c0().to_repr().as_ref().iter().rev());
    bytes.extend_from_slice(&[0u8; 16]);
    bytes.extend(y.c1().to_repr().as_ref().iter().rev());
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
    Bls12381::multi_miller_loop(&[(&g1, &g2)]).pow_vartime(definition_exponent())
}

#[cfg(test)]
pub(crate) fn definition_value_power(g1: G1Affine, g2: G2Affine, exponent: u64) -> Fq12 {
    definition_value(g1, g2).pow_vartime([exponent])
}

#[cfg(test)]
pub(crate) fn eip_2537_doubled_g1_generator() -> G1Affine {
    (eip_2537_g1_generator().to_curve() + eip_2537_g1_generator().to_curve()).to_affine()
}

#[cfg(test)]
pub(crate) fn corrected_generator_pairing() -> Gt {
    let pairing = Bls12381::pairing(&G1Affine::generator(), &G2Affine::generator());
    let Some(inverse) = Option::<Fr>::from(Fr::from(3u64).invert()) else {
        panic!("three is invertible")
    };
    pairing * inverse
}
