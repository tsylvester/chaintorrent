#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use super::interface::{
    PossessionG1Statement, PossessionG1StatementDescription,
    PossessionG1StatementDescriptionConstructorParams, PossessionG2Statement,
    PossessionG2StatementDescription, PossessionG2StatementDescriptionConstructorParams,
};
use encoding::{
    CanonicalFieldValue, CanonicalFields, CanonicalFieldsOverrides, build_canonical_fields,
};
use pairing::{
    AddG1Params, AddG2Params, EncodeG1Params, EncodeG1PayloadOverrides, EncodeG2Params,
    EncodeG2PayloadOverrides, G1GeneratorParams, G1GeneratorPayload, G2GeneratorParams,
    G2GeneratorPayload, IPairingAdapter, build_add_g1_payload, build_add_g2_payload,
    build_encode_g1_payload, build_encode_g2_payload,
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
        let Ok(success) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
        success.point
    });
    let commitment = overrides.commitment.unwrap_or_else(|| {
        let Ok(success) = pairing.add_g1(
            AddG1Params,
            build_add_g1_payload(pairing, Default::default()),
        );
        success.sum
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
        let Ok(success) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
        success.point
    });
    let commitment = overrides.commitment.unwrap_or_else(|| {
        let Ok(success) = pairing.add_g2(
            AddG2Params,
            build_add_g2_payload(pairing, Default::default()),
        );
        success.sum
    });
    PossessionG2Statement { key, commitment }
}

#[derive(Default)]
pub struct PossessionG1StatementCorruptions {
    pub key: Option<CanonicalFieldValue>,
    pub commitment: Option<CanonicalFieldValue>,
}

pub fn invalidate_possession_g1_statement<P: IPairingAdapter>(
    pairing: &P,
    corruptions: PossessionG1StatementCorruptions,
) -> CanonicalFields {
    let statement = build_possession_g1_statement(pairing, Default::default());
    let Ok(key) = pairing.encode_g1(
        EncodeG1Params,
        build_encode_g1_payload(
            pairing,
            EncodeG1PayloadOverrides {
                point: Some(statement.key),
            },
        ),
    );
    let Ok(commitment) = pairing.encode_g1(
        EncodeG1Params,
        build_encode_g1_payload(
            pairing,
            EncodeG1PayloadOverrides {
                point: Some(statement.commitment),
            },
        ),
    );
    let values = vec![
        corruptions
            .key
            .unwrap_or_else(|| CanonicalFieldValue::Bytes(key.bytes.as_ref().to_vec())),
        corruptions
            .commitment
            .unwrap_or_else(|| CanonicalFieldValue::Bytes(commitment.bytes.as_ref().to_vec())),
    ];
    build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(values),
    })
}

#[derive(Default)]
pub struct PossessionG2StatementCorruptions {
    pub key: Option<CanonicalFieldValue>,
    pub commitment: Option<CanonicalFieldValue>,
}

pub fn invalidate_possession_g2_statement<P: IPairingAdapter>(
    pairing: &P,
    corruptions: PossessionG2StatementCorruptions,
) -> CanonicalFields {
    let statement = build_possession_g2_statement(pairing, Default::default());
    let Ok(key) = pairing.encode_g2(
        EncodeG2Params,
        build_encode_g2_payload(
            pairing,
            EncodeG2PayloadOverrides {
                point: Some(statement.key),
            },
        ),
    );
    let Ok(commitment) = pairing.encode_g2(
        EncodeG2Params,
        build_encode_g2_payload(
            pairing,
            EncodeG2PayloadOverrides {
                point: Some(statement.commitment),
            },
        ),
    );
    let values = vec![
        corruptions
            .key
            .unwrap_or_else(|| CanonicalFieldValue::Bytes(key.bytes.as_ref().to_vec())),
        corruptions
            .commitment
            .unwrap_or_else(|| CanonicalFieldValue::Bytes(commitment.bytes.as_ref().to_vec())),
    ];
    build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(values),
    })
}

#[derive(Default)]
pub struct PossessionG1StatementDescriptionConstructorParamsOverrides;

pub fn build_possession_g1_statement_description_constructor_params<'a, P: IPairingAdapter>(
    pairing: &'a P,
    _overrides: PossessionG1StatementDescriptionConstructorParamsOverrides,
) -> PossessionG1StatementDescriptionConstructorParams<'a, P> {
    PossessionG1StatementDescriptionConstructorParams { pairing }
}

pub fn build_possession_g1_statement_description<'a, P: IPairingAdapter>(
    pairing: &'a P,
    overrides: PossessionG1StatementDescriptionConstructorParamsOverrides,
) -> PossessionG1StatementDescription<'a, P> {
    let Ok(description) = PossessionG1StatementDescription::try_new(
        build_possession_g1_statement_description_constructor_params(pairing, overrides),
    );
    description
}

#[derive(Default)]
pub struct PossessionG2StatementDescriptionConstructorParamsOverrides;

pub fn build_possession_g2_statement_description_constructor_params<'a, P: IPairingAdapter>(
    pairing: &'a P,
    _overrides: PossessionG2StatementDescriptionConstructorParamsOverrides,
) -> PossessionG2StatementDescriptionConstructorParams<'a, P> {
    PossessionG2StatementDescriptionConstructorParams { pairing }
}

pub fn build_possession_g2_statement_description<'a, P: IPairingAdapter>(
    pairing: &'a P,
    overrides: PossessionG2StatementDescriptionConstructorParamsOverrides,
) -> PossessionG2StatementDescription<'a, P> {
    let Ok(description) = PossessionG2StatementDescription::try_new(
        build_possession_g2_statement_description_constructor_params(pairing, overrides),
    );
    description
}
