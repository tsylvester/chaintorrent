use core::convert::Infallible;

pub struct Bls12381ArkworksPairing {
    /// The inverse of three in the scalar field, the multiple by which the
    /// library's reduced pairing exceeds the identifier's exact value.
    pub(super) reduced_pairing_correction: ark_bls12_381::Fr,
}

pub struct Bls12381ArkworksPairingConstructorParams;

pub type Bls12381ArkworksPairingTryNewReturn = Result<Bls12381ArkworksPairing, Infallible>;

#[derive(Clone)]
pub struct Bls12381ArkworksScalar {
    pub(super) value: ark_bls12_381::Fr,
}

#[derive(Clone)]
pub struct Bls12381ArkworksG1 {
    pub(super) value: ark_bls12_381::G1Affine,
}

#[derive(Clone)]
pub struct Bls12381ArkworksG2 {
    pub(super) value: ark_bls12_381::G2Affine,
}

pub struct Bls12381ArkworksGt {
    pub(super) value: ark_ec::pairing::PairingOutput<ark_bls12_381::Bls12_381>,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Bls12381ArkworksEncodedG1 {
    pub(super) bytes: [u8; 128],
}

#[derive(Clone, PartialEq, Eq)]
pub struct Bls12381ArkworksEncodedG2 {
    pub(super) bytes: [u8; 256],
}

pub struct Bls12381ArkworksEncodedScalar {
    pub(super) bytes: [u8; 32],
}

pub struct Bls12381ArkworksEncodedGt {
    pub(super) bytes: [u8; 576],
}
