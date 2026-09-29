#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{Bn254ArkworksG1, Bn254ArkworksG2, Bn254ArkworksScalar};
use ark_bn254::{Fr, G1Affine, G2Affine};
use ark_ec::AffineRepr;

impl Default for Bn254ArkworksScalar {
    fn default() -> Self {
        Bn254ArkworksScalar {
            value: Fr::from(1u64),
        }
    }
}

impl Default for Bn254ArkworksG1 {
    fn default() -> Self {
        Bn254ArkworksG1 {
            value: G1Affine::generator(),
        }
    }
}

impl Default for Bn254ArkworksG2 {
    fn default() -> Self {
        Bn254ArkworksG2 {
            value: G2Affine::generator(),
        }
    }
}
