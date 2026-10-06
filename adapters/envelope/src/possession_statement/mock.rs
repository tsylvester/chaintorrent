#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{PossessionG1Statement, PossessionG2Statement};
use pairing::{
    AddG1Params, AddG1Payload, AddG2Params, AddG2Payload, G1GeneratorParams, G1GeneratorPayload,
    G2GeneratorParams, G2GeneratorPayload, IPairingAdapter,
};

pub struct PossessionG1StatementOverrides<P: IPairingAdapter> {
    pub key: Option<P::G1>,
    pub commitment: Option<P::G1>,
}

impl<P: IPairingAdapter> Default for PossessionG1StatementOverrides<P> {
    fn default() -> Self {
        Self {
            key: None,
            commitment: None,
        }
    }
}

pub fn build_possession_g1_statement<P: IPairingAdapter>(
    pairing: &P,
    overrides: PossessionG1StatementOverrides<P>,
) -> PossessionG1Statement<P> {
    let key = overrides.key.unwrap_or_else(|| {
        let Ok(generator) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
        generator.point
    });
    let commitment = overrides.commitment.unwrap_or_else(|| {
        let Ok(generator) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
        let Ok(sum) = pairing.add_g1(
            AddG1Params,
            AddG1Payload {
                left: generator.point.clone(),
                right: generator.point,
            },
        );
        sum.sum
    });
    PossessionG1Statement { key, commitment }
}

pub struct PossessionG2StatementOverrides<P: IPairingAdapter> {
    pub key: Option<P::G2>,
    pub commitment: Option<P::G2>,
}

impl<P: IPairingAdapter> Default for PossessionG2StatementOverrides<P> {
    fn default() -> Self {
        Self {
            key: None,
            commitment: None,
        }
    }
}

pub fn build_possession_g2_statement<P: IPairingAdapter>(
    pairing: &P,
    overrides: PossessionG2StatementOverrides<P>,
) -> PossessionG2Statement<P> {
    let key = overrides.key.unwrap_or_else(|| {
        let Ok(generator) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
        generator.point
    });
    let commitment = overrides.commitment.unwrap_or_else(|| {
        let Ok(generator) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
        let Ok(sum) = pairing.add_g2(
            AddG2Params,
            AddG2Payload {
                left: generator.point.clone(),
                right: generator.point,
            },
        );
        sum.sum
    });
    PossessionG2Statement { key, commitment }
}
