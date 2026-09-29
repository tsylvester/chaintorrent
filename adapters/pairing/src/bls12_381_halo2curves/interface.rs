use core::convert::Infallible;

pub struct Bls12381Halo2curvesPairing;

pub struct Bls12381Halo2curvesPairingConstructorParams;

pub type Bls12381Halo2curvesPairingTryNewReturn = Result<Bls12381Halo2curvesPairing, Infallible>;

#[derive(Clone)]
pub struct Bls12381Halo2curvesScalar {
    pub(super) value: halo2curves::bls12381::Fr,
}

#[derive(Clone)]
pub struct Bls12381Halo2curvesG1 {
    pub(super) value: halo2curves::bls12381::G1Affine,
}

#[derive(Clone)]
pub struct Bls12381Halo2curvesG2 {
    pub(super) value: halo2curves::bls12381::G2Affine,
}
