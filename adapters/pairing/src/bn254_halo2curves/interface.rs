use core::convert::Infallible;

pub struct Bn254Halo2curvesPairing;

pub struct Bn254Halo2curvesPairingConstructorParams;

pub type Bn254Halo2curvesPairingTryNewReturn = Result<Bn254Halo2curvesPairing, Infallible>;

#[derive(Clone)]
pub struct Bn254Halo2curvesScalar {
    pub(super) value: halo2curves::bn256::Fr,
}

#[derive(Clone)]
pub struct Bn254Halo2curvesG1 {
    pub(super) value: halo2curves::bn256::G1Affine,
}

#[derive(Clone)]
pub struct Bn254Halo2curvesG2 {
    pub(super) value: halo2curves::bn256::G2Affine,
}

pub struct Bn254Halo2curvesGt {
    pub(super) value: halo2curves::bn256::Gt,
}
