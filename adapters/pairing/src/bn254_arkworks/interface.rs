use core::convert::Infallible;

pub struct Bn254ArkworksPairing;

pub struct Bn254ArkworksPairingConstructorParams;

pub type Bn254ArkworksPairingTryNewReturn = Result<Bn254ArkworksPairing, Infallible>;

#[derive(Clone)]
pub struct Bn254ArkworksScalar {
    pub(super) value: ark_bn254::Fr,
}

#[derive(Clone)]
pub struct Bn254ArkworksG1 {
    pub(super) value: ark_bn254::G1Affine,
}

#[derive(Clone)]
pub struct Bn254ArkworksG2 {
    pub(super) value: ark_bn254::G2Affine,
}
