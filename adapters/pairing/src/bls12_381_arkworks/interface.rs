use core::convert::Infallible;

pub struct Bls12381ArkworksPairing;

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
