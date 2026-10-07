#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::super::group_vectors::provides::{VectorCount, build_vector_count};
use super::interface::{
    PossessionVector, PossessionVectorDescription, PossessionVectorDescriptionConstructorParams,
    PossessionVectorsParams,
};

#[derive(Default)]
pub struct PossessionVectorOverrides {
    pub x: Option<[u8; 32]>,
    pub y: Option<[u8; 32]>,
    pub pk1: Option<Vec<u8>>,
    pub pk2: Option<Vec<u8>>,
    pub r1: Option<Vec<u8>>,
    pub z1: Option<[u8; 32]>,
    pub r2: Option<Vec<u8>>,
    pub z2: Option<[u8; 32]>,
}

pub fn build_possession_vector(overrides: PossessionVectorOverrides) -> PossessionVector {
    PossessionVector {
        x: overrides.x.unwrap_or([0x01; 32]),
        y: overrides.y.unwrap_or([0x02; 32]),
        pk1: overrides.pk1.unwrap_or_else(|| vec![0x03; 64]),
        pk2: overrides.pk2.unwrap_or_else(|| vec![0x04; 128]),
        r1: overrides.r1.unwrap_or_else(|| vec![0x05; 64]),
        z1: overrides.z1.unwrap_or([0x06; 32]),
        r2: overrides.r2.unwrap_or_else(|| vec![0x07; 128]),
        z2: overrides.z2.unwrap_or([0x08; 32]),
    }
}

#[derive(Default)]
pub struct PossessionVectorsParamsOverrides {
    pub count: Option<VectorCount>,
}

pub fn build_possession_vectors_params(
    overrides: PossessionVectorsParamsOverrides,
) -> PossessionVectorsParams {
    PossessionVectorsParams {
        count: overrides
            .count
            .unwrap_or_else(|| build_vector_count(Default::default())),
    }
}

pub fn build_possession_vector_description() -> PossessionVectorDescription {
    let Ok(description) =
        PossessionVectorDescription::try_new(PossessionVectorDescriptionConstructorParams);
    description
}
