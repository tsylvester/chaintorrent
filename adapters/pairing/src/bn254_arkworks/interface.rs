use core::convert::Infallible;

pub struct Bn254ArkworksPairing {
    /// The inverse in the scalar field of the multiple by which the library's
    /// reduced pairing exceeds the identifier's exact value,
    /// `2x(6x^2 + 3x + 1)` for the curve seed `x`.
    pub(super) reduced_pairing_correction: ark_bn254::Fr,
}

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

pub struct Bn254ArkworksGt {
    pub(super) value: ark_ec::pairing::PairingOutput<ark_bn254::Bn254>,
}
