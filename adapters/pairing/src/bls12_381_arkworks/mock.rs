#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    Bls12381ArkworksG1, Bls12381ArkworksG2, Bls12381ArkworksGt, Bls12381ArkworksScalar,
};
use ark_bls12_381::{Bls12_381, Fr, G1Affine, G2Affine};
use ark_ec::{AffineRepr, pairing::Pairing};

impl Default for Bls12381ArkworksScalar {
    fn default() -> Self {
        Bls12381ArkworksScalar {
            value: Fr::from(1u64),
        }
    }
}

impl Default for Bls12381ArkworksG1 {
    fn default() -> Self {
        Bls12381ArkworksG1 {
            value: G1Affine::generator(),
        }
    }
}

impl Default for Bls12381ArkworksG2 {
    fn default() -> Self {
        Bls12381ArkworksG2 {
            value: G2Affine::generator(),
        }
    }
}

impl Default for Bls12381ArkworksGt {
    fn default() -> Self {
        Bls12381ArkworksGt {
            value: Bls12_381::pairing(G1Affine::generator(), G2Affine::generator()),
        }
    }
}
