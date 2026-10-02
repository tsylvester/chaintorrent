#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    Bls12381Halo2curvesG1, Bls12381Halo2curvesG2, Bls12381Halo2curvesGt, Bls12381Halo2curvesScalar,
};
use halo2curves::bls12381::{Bls12381, Fr, G1Affine, G2Affine};
use halo2curves::ff::Field;
use halo2curves::pairing::Engine;

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
