#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    Bn254Halo2curvesG1, Bn254Halo2curvesG2, Bn254Halo2curvesGt, Bn254Halo2curvesScalar,
};
use halo2curves::bn256::{Bn256, Fr, G1Affine, G2Affine};
use halo2curves::ff::Field;
use halo2curves::pairing::Engine;

impl Default for Bn254Halo2curvesScalar {
    fn default() -> Self {
        Bn254Halo2curvesScalar { value: Fr::ONE }
    }
}

impl Default for Bn254Halo2curvesG1 {
    fn default() -> Self {
        Bn254Halo2curvesG1 {
            value: G1Affine::generator(),
        }
    }
}

impl Default for Bn254Halo2curvesG2 {
    fn default() -> Self {
        Bn254Halo2curvesG2 {
            value: G2Affine::generator(),
        }
    }
}

impl Default for Bn254Halo2curvesGt {
    fn default() -> Self {
        Bn254Halo2curvesGt {
            value: Bn256::pairing(&G1Affine::generator(), &G2Affine::generator()),
        }
    }
}
