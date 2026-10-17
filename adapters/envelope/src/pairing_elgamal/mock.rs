#![allow(dead_code, clippy::expect_used, clippy::too_many_arguments)]

use super::interface::{
    PairingElGamalEnvelope, PairingElGamalKeyAgreement,
    PairingElGamalKeyAgreementConstructorParams, PairingElGamalKeyPair, PairingElGamalPossession,
    PairingElGamalPublicKeys,
};
use crate::factory::provides::{
    EnvelopeComponentsOverrides, KeyPairComponentsOverrides, PublicKeysComponentsOverrides,
    build_envelope_components, build_key_pair_components, build_public_keys_components,
};
use crate::possession_statement::provides::{
    PossessionG1StatementOverrides, PossessionG2StatementOverrides, build_possession_g1_statement,
    build_possession_g2_statement,
};
use domain::{Secret, SecretConstructorParamsOverrides, build_secret};
use encoding::{EncodeParams, IEncoderAdapter};
use hash_to_scalar::{HashToScalarParams, HashToScalarPayload, IHashToScalarAdapter};
use pairing::{
    AddScalarParams, AddScalarPayloadOverrides, G1GeneratorParams, G1GeneratorPayload,
    G2GeneratorParams, G2GeneratorPayload, IPairingAdapter, IPairingArithmetic,
    ISampleUniformScalar, MulG1Params, MulG1PayloadOverrides, MulG2Params, MulG2PayloadOverrides,
    MulScalarParams, MulScalarPayloadOverrides, SampleUniformScalarParams,
    SampleUniformScalarPayloadOverrides, build_add_scalar_payload, build_mul_g1_payload,
    build_mul_g2_payload, build_mul_scalar_payload, build_sample_uniform_scalar_payload,
};
use random::IRandomSourceAdapter;

#[derive(Default)]
pub struct PairingElGamalKeyAgreementConstructorParamsOverrides;

pub fn build_pairing_elgamal_key_agreement_constructor_params<
    'a,
    P: IPairingArithmetic,
    E: IEncoderAdapter,
>(
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    _overrides: PairingElGamalKeyAgreementConstructorParamsOverrides,
) -> PairingElGamalKeyAgreementConstructorParams<'a, P, E> {
    PairingElGamalKeyAgreementConstructorParams {
        pairing,
        hash_to_scalar,
        encoder,
        random,
    }
}

pub fn build_pairing_elgamal_key_agreement<'a, P: IPairingArithmetic, E: IEncoderAdapter>(
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    overrides: PairingElGamalKeyAgreementConstructorParamsOverrides,
) -> PairingElGamalKeyAgreement<'a, P, E> {
    PairingElGamalKeyAgreement::try_new(build_pairing_elgamal_key_agreement_constructor_params(
        pairing,
        hash_to_scalar,
        encoder,
        random,
        overrides,
    ))
    .expect("the possession tags are visible ASCII of an admitted length")
}

fn sampled_scalar<P: IPairingAdapter>(fill: u8) -> P::Scalar {
    P::Scalar::sample_from_uniform_bytes(
        SampleUniformScalarParams,
        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
            uniform: Some(build_secret(
                vec![fill; <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH],
                SecretConstructorParamsOverrides,
            )),
        }),
    )
    .expect("sampling a constant scalar always succeeds")
    .scalar
    .expose()
    .clone()
}

fn challenge_g1<P: IPairingArithmetic, E: IEncoderAdapter>(
    adapter: &PairingElGamalKeyAgreement<'_, P, E>,
    key: &P::G1,
    commitment: &P::G1,
) -> P::Scalar {
    let encoded = adapter
        .encoder
        .encode(
            EncodeParams {
                description: &adapter.g1_statement_description,
            },
            &build_possession_g1_statement(
                adapter.pairing,
                PossessionG1StatementOverrides {
                    key: Some(key.clone()),
                    commitment: Some(commitment.clone()),
                },
            ),
        )
        .expect("encoding a possession statement always succeeds");
    adapter
        .hash_to_scalar
        .hash_to_scalar(
            HashToScalarParams {
                tag: &adapter.g1_tag,
            },
            HashToScalarPayload {
                message: &encoded.bytes,
            },
        )
        .expect("hashing a possession challenge always succeeds")
        .scalar
}

fn challenge_g2<P: IPairingArithmetic, E: IEncoderAdapter>(
    adapter: &PairingElGamalKeyAgreement<'_, P, E>,
    key: &P::G2,
    commitment: &P::G2,
) -> P::Scalar {
    let encoded = adapter
        .encoder
        .encode(
            EncodeParams {
                description: &adapter.g2_statement_description,
            },
            &build_possession_g2_statement(
                adapter.pairing,
                PossessionG2StatementOverrides {
                    key: Some(key.clone()),
                    commitment: Some(commitment.clone()),
                },
            ),
        )
        .expect("encoding a possession statement always succeeds");
    adapter
        .hash_to_scalar
        .hash_to_scalar(
            HashToScalarParams {
                tag: &adapter.g2_tag,
            },
            HashToScalarPayload {
                message: &encoded.bytes,
            },
        )
        .expect("hashing a possession challenge always succeeds")
        .scalar
}

pub struct PairingElGamalKeyPairOverrides<P: IPairingAdapter> {
    pub x: Option<Secret<P::Scalar>>,
    pub y: Option<Secret<P::Scalar>>,
    pub pk1: Option<P::G1>,
    pub pk2: Option<P::G2>,
}

impl<P: IPairingAdapter> Default for PairingElGamalKeyPairOverrides<P> {
    fn default() -> Self {
        Self {
            x: None,
            y: None,
            pk1: None,
            pk2: None,
        }
    }
}

pub fn build_pairing_elgamal_key_pair<P: IPairingAdapter>(
    pairing: &P,
    overrides: PairingElGamalKeyPairOverrides<P>,
) -> PairingElGamalKeyPair<P> {
    let components = build_key_pair_components(
        pairing,
        KeyPairComponentsOverrides {
            x: overrides.x,
            y: overrides.y,
        },
    );
    let pk1 = overrides.pk1.unwrap_or_else(|| {
        let Ok(generator) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
        let Ok(product) = pairing.mul_g1(
            MulG1Params,
            build_mul_g1_payload(
                pairing,
                MulG1PayloadOverrides {
                    point: Some(generator.point),
                    scalar: Some(components.x.expose().clone()),
                },
            ),
        );
        product.product
    });
    let pk2 = overrides.pk2.unwrap_or_else(|| {
        let Ok(generator) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
        let Ok(product) = pairing.mul_g2(
            MulG2Params,
            build_mul_g2_payload(
                pairing,
                MulG2PayloadOverrides {
                    point: Some(generator.point),
                    scalar: Some(components.y.expose().clone()),
                },
            ),
        );
        product.product
    });
    PairingElGamalKeyPair {
        x: components.x,
        y: components.y,
        pk1,
        pk2,
    }
}

pub struct PairingElGamalPublicKeysOverrides<P: IPairingAdapter> {
    pub pk1: Option<P::G1>,
    pub pk2: Option<P::G2>,
}

impl<P: IPairingAdapter> Default for PairingElGamalPublicKeysOverrides<P> {
    fn default() -> Self {
        Self {
            pk1: None,
            pk2: None,
        }
    }
}

pub fn build_pairing_elgamal_public_keys<P: IPairingAdapter>(
    pairing: &P,
    overrides: PairingElGamalPublicKeysOverrides<P>,
) -> PairingElGamalPublicKeys<P> {
    let components = build_public_keys_components(
        pairing,
        PublicKeysComponentsOverrides {
            pk1: overrides.pk1,
            pk2: overrides.pk2,
        },
    );
    PairingElGamalPublicKeys {
        pk1: components.pk1,
        pk2: components.pk2,
    }
}

pub struct PairingElGamalEnvelopeOverrides<P: IPairingAdapter> {
    pub c1: Option<P::G1>,
    pub c2: Option<P::G1>,
    pub d1: Option<P::G2>,
    pub d2: Option<P::G2>,
}

impl<P: IPairingAdapter> Default for PairingElGamalEnvelopeOverrides<P> {
    fn default() -> Self {
        Self {
            c1: None,
            c2: None,
            d1: None,
            d2: None,
        }
    }
}

pub fn build_pairing_elgamal_envelope<P: IPairingAdapter>(
    pairing: &P,
    overrides: PairingElGamalEnvelopeOverrides<P>,
) -> PairingElGamalEnvelope<P> {
    let components = build_envelope_components(
        pairing,
        EnvelopeComponentsOverrides {
            c1: overrides.c1,
            c2: overrides.c2,
            d1: overrides.d1,
            d2: overrides.d2,
        },
    );
    PairingElGamalEnvelope {
        c1: components.c1,
        c2: components.c2,
        d1: components.d1,
        d2: components.d2,
    }
}

pub struct PairingElGamalPossessionOverrides<P: IPairingAdapter> {
    pub r1: Option<P::G1>,
    pub z1: Option<P::Scalar>,
    pub r2: Option<P::G2>,
    pub z2: Option<P::Scalar>,
}

impl<P: IPairingAdapter> Default for PairingElGamalPossessionOverrides<P> {
    fn default() -> Self {
        Self {
            r1: None,
            z1: None,
            r2: None,
            z2: None,
        }
    }
}

pub fn build_pairing_elgamal_possession<P: IPairingArithmetic, E: IEncoderAdapter>(
    adapter: &PairingElGamalKeyAgreement<'_, P, E>,
    key_pair: &PairingElGamalKeyPair<P>,
    overrides: PairingElGamalPossessionOverrides<P>,
) -> PairingElGamalPossession<P> {
    let pairing = adapter.pairing;
    let k1 = sampled_scalar::<P>(0x0B);
    let k2 = sampled_scalar::<P>(0x0C);
    let Ok(g1) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
    let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
    let Ok(r1) = pairing.mul_g1(
        MulG1Params,
        build_mul_g1_payload(
            pairing,
            MulG1PayloadOverrides {
                point: Some(g1.point),
                scalar: Some(k1.clone()),
            },
        ),
    );
    let r1 = r1.product;
    let Ok(r2) = pairing.mul_g2(
        MulG2Params,
        build_mul_g2_payload(
            pairing,
            MulG2PayloadOverrides {
                point: Some(g2.point),
                scalar: Some(k2.clone()),
            },
        ),
    );
    let r2 = r2.product;
    let c1 = challenge_g1(adapter, &key_pair.pk1, &r1);
    let c2 = challenge_g2(adapter, &key_pair.pk2, &r2);
    let Ok(c1_x) = pairing.mul_scalar(
        MulScalarParams,
        build_mul_scalar_payload(
            pairing,
            MulScalarPayloadOverrides {
                left: Some(c1),
                right: Some(key_pair.x.expose().clone()),
            },
        ),
    );
    let Ok(z1) = pairing.add_scalar(
        AddScalarParams,
        build_add_scalar_payload(
            pairing,
            AddScalarPayloadOverrides {
                left: Some(k1),
                right: Some(c1_x.product),
            },
        ),
    );
    let Ok(c2_y) = pairing.mul_scalar(
        MulScalarParams,
        build_mul_scalar_payload(
            pairing,
            MulScalarPayloadOverrides {
                left: Some(c2),
                right: Some(key_pair.y.expose().clone()),
            },
        ),
    );
    let Ok(z2) = pairing.add_scalar(
        AddScalarParams,
        build_add_scalar_payload(
            pairing,
            AddScalarPayloadOverrides {
                left: Some(k2),
                right: Some(c2_y.product),
            },
        ),
    );
    PairingElGamalPossession {
        r1: overrides.r1.unwrap_or(r1),
        z1: overrides.z1.unwrap_or(z1.sum),
        r2: overrides.r2.unwrap_or(r2),
        z2: overrides.z2.unwrap_or(z2.sum),
    }
}
