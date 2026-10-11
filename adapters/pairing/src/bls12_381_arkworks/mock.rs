#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    Bls12381ArkworksEncodedGt, Bls12381ArkworksEncodedScalar, Bls12381ArkworksG1,
    Bls12381ArkworksG2, Bls12381ArkworksGt, Bls12381ArkworksPairing,
    Bls12381ArkworksPairingConstructorParams, Bls12381ArkworksScalar,
};
use ark_bls12_381::{Bls12_381, Fr, G1Affine, G2Affine};
#[cfg(test)]
use ark_bls12_381::{Fq, Fq2, Fq12};
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
pub(crate) struct Bls12381ArkworksScalarOverrides {
    pub value: Option<Fr>,
}

pub(crate) fn build_bls12_381_arkworks_scalar(
    overrides: Bls12381ArkworksScalarOverrides,
) -> Bls12381ArkworksScalar {
    Bls12381ArkworksScalar {
        value: overrides.value.unwrap_or_else(|| Fr::from(1u64)),
    }
}

#[derive(Default)]
pub(crate) struct Bls12381ArkworksG1Overrides {
    pub value: Option<G1Affine>,
}

pub(crate) fn build_bls12_381_arkworks_g1(
    overrides: Bls12381ArkworksG1Overrides,
) -> Bls12381ArkworksG1 {
    Bls12381ArkworksG1 {
        value: overrides.value.unwrap_or_else(G1Affine::generator),
    }
}

#[derive(Default)]
pub(crate) struct Bls12381ArkworksG2Overrides {
    pub value: Option<G2Affine>,
}

pub(crate) fn build_bls12_381_arkworks_g2(
    overrides: Bls12381ArkworksG2Overrides,
) -> Bls12381ArkworksG2 {
    Bls12381ArkworksG2 {
        value: overrides.value.unwrap_or_else(G2Affine::generator),
    }
}

#[derive(Default)]
pub(crate) struct Bls12381ArkworksGtOverrides {
    pub value: Option<PairingOutput<Bls12_381>>,
}

pub(crate) fn build_bls12_381_arkworks_gt(
    overrides: Bls12381ArkworksGtOverrides,
) -> Bls12381ArkworksGt {
    Bls12381ArkworksGt {
        value: overrides
            .value
            .unwrap_or_else(|| Bls12_381::pairing(G1Affine::generator(), G2Affine::generator())),
    }
}

pub(crate) fn build_bls12_381_arkworks_pairing() -> Bls12381ArkworksPairing {
    let Ok(pairing) = Bls12381ArkworksPairing::try_new(Bls12381ArkworksPairingConstructorParams);
    pairing
}

#[derive(Default)]
pub(crate) struct Bls12381ArkworksEncodedScalarOverrides {
    pub bytes: Option<[u8; 32]>,
}

pub(crate) fn build_bls12_381_arkworks_encoded_scalar(
    overrides: Bls12381ArkworksEncodedScalarOverrides,
) -> Bls12381ArkworksEncodedScalar {
    Bls12381ArkworksEncodedScalar {
        bytes: overrides.bytes.unwrap_or([1u8; 32]),
    }
}

#[derive(Default)]
pub(crate) struct Bls12381ArkworksEncodedGtOverrides {
    pub bytes: Option<[u8; 576]>,
}

pub(crate) fn build_bls12_381_arkworks_encoded_gt(
    overrides: Bls12381ArkworksEncodedGtOverrides,
) -> Bls12381ArkworksEncodedGt {
    Bls12381ArkworksEncodedGt {
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
pub(crate) fn scalar_value(hex: &str) -> Fr {
    Fr::from_be_bytes_mod_order(&vector_bytes(hex))
}

#[cfg(test)]
pub(crate) fn requires_zeroize_on_drop<T: ZeroizeOnDrop>() {}

#[cfg(test)]
pub(crate) fn base_field_modulus() -> BigUint {
    BigUint::from_bytes_be(&vector_bytes(BASE_FIELD_MODULUS_PADDED_HEX))
}

#[cfg(test)]
pub(crate) fn gt_identity_encoding() -> Vec<u8> {
    let mut bytes = vec![0u8; 576];
    bytes[47] = 1;
    bytes
}

#[cfg(test)]
pub(crate) fn g1_point_from_encoding(bytes: &[u8]) -> G1Affine {
    G1Affine::new_unchecked(
        Fq::from_be_bytes_mod_order(&bytes[16..64]),
        Fq::from_be_bytes_mod_order(&bytes[80..128]),
    )
}

#[cfg(test)]
pub(crate) fn g2_point_from_encoding(bytes: &[u8]) -> G2Affine {
    let x_c0 = Fq::from_be_bytes_mod_order(&bytes[16..64]);
    let x_c1 = Fq::from_be_bytes_mod_order(&bytes[80..128]);
    let y_c0 = Fq::from_be_bytes_mod_order(&bytes[144..192]);
    let y_c1 = Fq::from_be_bytes_mod_order(&bytes[208..256]);
    G2Affine::new_unchecked(Fq2::new(x_c0, x_c1), Fq2::new(y_c0, y_c1))
}

#[cfg(test)]
pub(crate) fn eip_2537_g1_generator() -> G1Affine {
    g1_point_from_encoding(&vector_bytes(G1_GENERATOR_HEX))
}

#[cfg(test)]
pub(crate) fn eip_2537_negated_g1_generator() -> G1Affine {
    g1_point_from_encoding(&vector_bytes(NEG_G1_GENERATOR_HEX))
}

#[cfg(test)]
pub(crate) fn eip_2537_g2_generator() -> G2Affine {
    g2_point_from_encoding(&vector_bytes(G2_GENERATOR_HEX))
}

#[cfg(test)]
pub(crate) fn g1_outside_subgroup_bytes() -> Vec<u8> {
    let Some(point) = (1u64..).find_map(|c| {
        G1Affine::get_point_from_x_unchecked(Fq::from(c), false)
            .filter(|point| !point.is_in_correct_subgroup_assuming_on_curve())
    }) else {
        panic!("an on-curve point outside the subgroup exists")
    };
    let Some((x, y)) = point.xy() else {
        panic!("the point has coordinates")
    };
    let mut bytes = Vec::with_capacity(128);
    bytes.extend_from_slice(&[0u8; 16]);
    bytes.extend_from_slice(&x.into_bigint().to_bytes_be());
    bytes.extend_from_slice(&[0u8; 16]);
    bytes.extend_from_slice(&y.into_bigint().to_bytes_be());
    bytes
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
    let mut bytes = Vec::with_capacity(256);
    bytes.extend_from_slice(&[0u8; 16]);
    bytes.extend_from_slice(&x.c0.into_bigint().to_bytes_be());
    bytes.extend_from_slice(&[0u8; 16]);
    bytes.extend_from_slice(&x.c1.into_bigint().to_bytes_be());
    bytes.extend_from_slice(&[0u8; 16]);
    bytes.extend_from_slice(&y.c0.into_bigint().to_bytes_be());
    bytes.extend_from_slice(&[0u8; 16]);
    bytes.extend_from_slice(&y.c1.into_bigint().to_bytes_be());
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
    Bls12_381::multi_miller_loop([g1], [g2])
        .0
        .pow(definition_exponent())
}

#[cfg(test)]
pub(crate) fn definition_value_power(g1: G1Affine, g2: G2Affine, exponent: u64) -> Fq12 {
    definition_value(g1, g2).pow([exponent])
}

#[cfg(test)]
pub(crate) fn eip_2537_doubled_g1_generator() -> G1Affine {
    (eip_2537_g1_generator() + eip_2537_g1_generator()).into_affine()
}
