#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    DecodeTarget, RejectionVector, RejectionVectorDescription,
    RejectionVectorDescriptionConstructorParams,
};

#[derive(Default)]
pub struct RejectionVectorOverrides {
    pub target: Option<DecodeTarget>,
    pub input: Option<Vec<u8>>,
}

pub fn build_rejection_vector(overrides: RejectionVectorOverrides) -> RejectionVector {
    RejectionVector {
        target: overrides.target.unwrap_or(DecodeTarget::G2),
        input: overrides.input.unwrap_or_else(|| vec![0x01; 128]),
    }
}

pub fn build_rejection_vector_description() -> RejectionVectorDescription {
    let Ok(description) =
        RejectionVectorDescription::try_new(RejectionVectorDescriptionConstructorParams);
    description
}
