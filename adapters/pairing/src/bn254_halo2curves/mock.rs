#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{Bn254Halo2curvesG1, Bn254Halo2curvesG2, Bn254Halo2curvesScalar};
use halo2curves::bn256::{Fr, G1Affine, G2Affine};
use halo2curves::ff::Field;

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
