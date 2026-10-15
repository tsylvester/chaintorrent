#![allow(clippy::expect_used)]

use super::interface::{
    TransferFirstMessages, TransferStatement, TransferStatementDescription,
    TransferStatementDescriptionConstructorParams,
};
use crate::mint_statement::provides::TransferPurpose;
use chain::IChainForms;
use domain::{
    AssetIdentityHash, AssetIdentityHashConstructorParamsOverrides, ParameterSetIdentifier,
    ParameterSetIdentifierConstructorParamsOverrides, SecretConstructorParamsOverrides,
    SuiteIdentifier, SuiteIdentifierConstructorParamsOverrides, build_asset_identity_hash,
    build_parameter_set_identifier, build_secret, build_suite_identifier,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFields, CanonicalFieldsOverrides,
    FromFieldParamsOverrides, ICanonicalField, ToFieldParamsOverrides, build_canonical_fields,
    build_from_field_params, build_to_field_params,
};
use envelope::{
    EnvelopeComponents, EnvelopeComponentsOverrides, PublicKeysComponents,
    PublicKeysComponentsOverrides, build_envelope_components, build_public_keys_components,
};
use pairing::{
    EncodeG1Params, EncodeG1PayloadOverrides, EncodeG2Params, EncodeG2PayloadOverrides,
    G1GeneratorParams, G1GeneratorPayload, G2GeneratorParams, G2GeneratorPayload, IPairingAdapter,
    ISampleUniformScalar, MulG1Params, MulG1Payload, MulG2Params, MulG2Payload,
    SampleUniformScalarParams, SampleUniformScalarPayload, build_encode_g1_payload,
    build_encode_g2_payload,
};

pub struct TransferFirstMessagesOverrides<G1, G2> {
    pub pk1: Option<G1>,
    pub c1: Option<G1>,
    pub c2: Option<G1>,
    pub pk2: Option<G2>,
    pub d1: Option<G2>,
    pub d2: Option<G2>,
}

impl<G1, G2> Default for TransferFirstMessagesOverrides<G1, G2> {
    fn default() -> Self {
        Self {
            pk1: None,
            c1: None,
            c2: None,
            pk2: None,
            d1: None,
            d2: None,
        }
    }
}

pub fn build_transfer_first_messages<P: IPairingAdapter>(
    pairing: &P,
    overrides: TransferFirstMessagesOverrides<P::G1, P::G2>,
) -> TransferFirstMessages<P::G1, P::G2> {
    TransferFirstMessages {
        pk1: overrides.pk1.unwrap_or_else(|| g1_mul(pairing, 0x2D)),
        c1: overrides.c1.unwrap_or_else(|| g1_mul(pairing, 0x2E)),
        c2: overrides.c2.unwrap_or_else(|| g1_mul(pairing, 0x2F)),
        pk2: overrides.pk2.unwrap_or_else(|| g2_mul(pairing, 0x30)),
        d1: overrides.d1.unwrap_or_else(|| g2_mul(pairing, 0x31)),
        d2: overrides.d2.unwrap_or_else(|| g2_mul(pairing, 0x32)),
    }
}

pub struct TransferStatementOverrides<P: IPairingAdapter, F: IChainForms> {
    pub suite_identifier: Option<SuiteIdentifier>,
    pub chain: Option<F::ChainIdentifier>,
    pub entitlement_contract: Option<F::Identity>,
    pub asset_identity_hash: Option<AssetIdentityHash>,
    pub parameter_set_identifier: Option<ParameterSetIdentifier>,
    pub source_entitlement: Option<F::Entitlement>,
    pub target_entitlement: Option<F::Entitlement>,
    pub old_interval: Option<F::Interval>,
    pub new_interval: Option<F::Interval>,
    pub purpose: Option<TransferPurpose>,
    pub seller: Option<F::Identity>,
    pub buyer: Option<F::Identity>,
    pub seller_keys: Option<PublicKeysComponents<P::G1, P::G2>>,
    pub buyer_keys: Option<PublicKeysComponents<P::G1, P::G2>>,
    pub old_envelope: Option<EnvelopeComponents<P::G1, P::G2>>,
    pub new_envelope: Option<EnvelopeComponents<P::G1, P::G2>>,
    pub expiry: Option<u64>,
    pub first_messages: Option<TransferFirstMessages<P::G1, P::G2>>,
}

impl<P: IPairingAdapter, F: IChainForms> Default for TransferStatementOverrides<P, F> {
    fn default() -> Self {
        Self {
            suite_identifier: None,
            chain: None,
            entitlement_contract: None,
            asset_identity_hash: None,
            parameter_set_identifier: None,
            source_entitlement: None,
            target_entitlement: None,
            old_interval: None,
            new_interval: None,
            purpose: None,
            seller: None,
            buyer: None,
            seller_keys: None,
            buyer_keys: None,
            old_envelope: None,
            new_envelope: None,
            expiry: None,
            first_messages: None,
        }
    }
}

pub fn build_transfer_statement<P: IPairingAdapter, F: IChainForms>(
    pairing: &P,
    overrides: TransferStatementOverrides<P, F>,
) -> TransferStatement<P, F> {
    TransferStatement {
        suite_identifier: overrides.suite_identifier.unwrap_or_else(|| {
            build_suite_identifier(SuiteIdentifierConstructorParamsOverrides::default())
        }),
        chain: overrides
            .chain
            .unwrap_or_else(|| form_from_seed::<F::ChainIdentifier>(0x11)),
        entitlement_contract: overrides
            .entitlement_contract
            .unwrap_or_else(|| form_from_seed::<F::Identity>(0x12)),
        asset_identity_hash: overrides.asset_identity_hash.unwrap_or_else(|| {
            build_asset_identity_hash(AssetIdentityHashConstructorParamsOverrides::default())
        }),
        parameter_set_identifier: overrides.parameter_set_identifier.unwrap_or_else(|| {
            build_parameter_set_identifier(
                ParameterSetIdentifierConstructorParamsOverrides::default(),
            )
        }),
        source_entitlement: overrides
            .source_entitlement
            .unwrap_or_else(|| form_from_seed::<F::Entitlement>(0x13)),
        target_entitlement: overrides
            .target_entitlement
            .unwrap_or_else(|| form_from_seed::<F::Entitlement>(0x14)),
        old_interval: overrides
            .old_interval
            .unwrap_or_else(|| form_from_seed::<F::Interval>(0x15)),
        new_interval: overrides
            .new_interval
            .unwrap_or_else(|| form_from_seed::<F::Interval>(0x16)),
        purpose: overrides.purpose.unwrap_or(TransferPurpose::Transfer),
        seller: overrides
            .seller
            .unwrap_or_else(|| form_from_seed::<F::Identity>(0x17)),
        buyer: overrides
            .buyer
            .unwrap_or_else(|| form_from_seed::<F::Identity>(0x18)),
        seller_keys: overrides.seller_keys.unwrap_or_else(|| {
            build_public_keys_components(
                pairing,
                PublicKeysComponentsOverrides {
                    pk1: Some(g1_mul(pairing, 0x21)),
                    pk2: Some(g2_mul(pairing, 0x22)),
                },
            )
        }),
        buyer_keys: overrides.buyer_keys.unwrap_or_else(|| {
            build_public_keys_components(
                pairing,
                PublicKeysComponentsOverrides {
                    pk1: Some(g1_mul(pairing, 0x23)),
                    pk2: Some(g2_mul(pairing, 0x24)),
                },
            )
        }),
        old_envelope: overrides.old_envelope.unwrap_or_else(|| {
            build_envelope_components(
                pairing,
                EnvelopeComponentsOverrides {
                    c1: Some(g1_mul(pairing, 0x25)),
                    c2: Some(g1_mul(pairing, 0x26)),
                    d1: Some(g2_mul(pairing, 0x27)),
                    d2: Some(g2_mul(pairing, 0x28)),
                },
            )
        }),
        new_envelope: overrides.new_envelope.unwrap_or_else(|| {
            build_envelope_components(
                pairing,
                EnvelopeComponentsOverrides {
                    c1: Some(g1_mul(pairing, 0x29)),
                    c2: Some(g1_mul(pairing, 0x2A)),
                    d1: Some(g2_mul(pairing, 0x2B)),
                    d2: Some(g2_mul(pairing, 0x2C)),
                },
            )
        }),
        expiry: overrides.expiry.unwrap_or(1_700_000_000),
        first_messages: overrides
            .first_messages
            .unwrap_or_else(|| build_transfer_first_messages(pairing, Default::default())),
    }
}

#[derive(Default)]
pub struct TransferStatementCorruptions {
    pub suite_identifier: Option<Vec<CanonicalFieldValue>>,
    pub chain: Option<Vec<CanonicalFieldValue>>,
    pub entitlement_contract: Option<Vec<CanonicalFieldValue>>,
    pub asset_identity_hash: Option<Vec<CanonicalFieldValue>>,
    pub parameter_set_identifier: Option<Vec<CanonicalFieldValue>>,
    pub source_entitlement: Option<Vec<CanonicalFieldValue>>,
    pub target_entitlement: Option<Vec<CanonicalFieldValue>>,
    pub old_interval: Option<Vec<CanonicalFieldValue>>,
    pub new_interval: Option<Vec<CanonicalFieldValue>>,
    pub purpose: Option<Vec<CanonicalFieldValue>>,
    pub seller: Option<Vec<CanonicalFieldValue>>,
    pub buyer: Option<Vec<CanonicalFieldValue>>,
    pub seller_keys: Option<Vec<CanonicalFieldValue>>,
    pub buyer_keys: Option<Vec<CanonicalFieldValue>>,
    pub old_envelope: Option<Vec<CanonicalFieldValue>>,
    pub new_envelope: Option<Vec<CanonicalFieldValue>>,
    pub expiry: Option<Vec<CanonicalFieldValue>>,
    pub first_messages: Option<Vec<CanonicalFieldValue>>,
}

pub fn invalidate_transfer_statement<P: IPairingAdapter, F: IChainForms>(
    pairing: &P,
    corruptions: TransferStatementCorruptions,
) -> CanonicalFields {
    let baseline: TransferStatement<P, F> =
        build_transfer_statement(pairing, TransferStatementOverrides::default());
    let mut values: Vec<CanonicalFieldValue> = Vec::new();
    values.extend(corruptions.suite_identifier.unwrap_or_else(|| {
        vec![
            CanonicalFieldValue::FixedBytes32(*baseline.suite_identifier.identifier()),
            CanonicalFieldValue::Unsigned16(baseline.suite_identifier.version()),
        ]
    }));
    values.extend(
        corruptions
            .chain
            .unwrap_or_else(|| vec![render_form(&baseline.chain)]),
    );
    values.extend(
        corruptions
            .entitlement_contract
            .unwrap_or_else(|| vec![render_form(&baseline.entitlement_contract)]),
    );
    values.extend(corruptions.asset_identity_hash.unwrap_or_else(|| {
        vec![CanonicalFieldValue::FixedBytes32(
            *baseline.asset_identity_hash.as_bytes(),
        )]
    }));
    values.extend(corruptions.parameter_set_identifier.unwrap_or_else(|| {
        vec![CanonicalFieldValue::FixedBytes32(
            *baseline.parameter_set_identifier.as_bytes(),
        )]
    }));
    values.extend(
        corruptions
            .source_entitlement
            .unwrap_or_else(|| vec![render_form(&baseline.source_entitlement)]),
    );
    values.extend(
        corruptions
            .target_entitlement
            .unwrap_or_else(|| vec![render_form(&baseline.target_entitlement)]),
    );
    values.extend(
        corruptions
            .old_interval
            .unwrap_or_else(|| vec![render_form(&baseline.old_interval)]),
    );
    values.extend(
        corruptions
            .new_interval
            .unwrap_or_else(|| vec![render_form(&baseline.new_interval)]),
    );
    values.extend(
        corruptions
            .purpose
            .unwrap_or_else(|| vec![CanonicalFieldValue::Unsigned16(baseline.purpose.code())]),
    );
    values.extend(
        corruptions
            .seller
            .unwrap_or_else(|| vec![render_form(&baseline.seller)]),
    );
    values.extend(
        corruptions
            .buyer
            .unwrap_or_else(|| vec![render_form(&baseline.buyer)]),
    );
    values.extend(corruptions.seller_keys.unwrap_or_else(|| {
        vec![
            encode_g1_field(pairing, &baseline.seller_keys.pk1),
            encode_g2_field(pairing, &baseline.seller_keys.pk2),
        ]
    }));
    values.extend(corruptions.buyer_keys.unwrap_or_else(|| {
        vec![
            encode_g1_field(pairing, &baseline.buyer_keys.pk1),
            encode_g2_field(pairing, &baseline.buyer_keys.pk2),
        ]
    }));
    values.extend(corruptions.old_envelope.unwrap_or_else(|| {
        vec![
            encode_g1_field(pairing, &baseline.old_envelope.c1),
            encode_g1_field(pairing, &baseline.old_envelope.c2),
            encode_g2_field(pairing, &baseline.old_envelope.d1),
            encode_g2_field(pairing, &baseline.old_envelope.d2),
        ]
    }));
    values.extend(corruptions.new_envelope.unwrap_or_else(|| {
        vec![
            encode_g1_field(pairing, &baseline.new_envelope.c1),
            encode_g1_field(pairing, &baseline.new_envelope.c2),
            encode_g2_field(pairing, &baseline.new_envelope.d1),
            encode_g2_field(pairing, &baseline.new_envelope.d2),
        ]
    }));
    values.extend(
        corruptions
            .expiry
            .unwrap_or_else(|| vec![CanonicalFieldValue::Unsigned64(baseline.expiry)]),
    );
    values.extend(corruptions.first_messages.unwrap_or_else(|| {
        vec![
            encode_g1_field(pairing, &baseline.first_messages.pk1),
            encode_g1_field(pairing, &baseline.first_messages.c1),
            encode_g1_field(pairing, &baseline.first_messages.c2),
            encode_g2_field(pairing, &baseline.first_messages.pk2),
            encode_g2_field(pairing, &baseline.first_messages.d1),
            encode_g2_field(pairing, &baseline.first_messages.d2),
        ]
    }));
    build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(values),
    })
}

#[derive(Default)]
pub struct TransferStatementDescriptionConstructorParamsOverrides;

pub fn build_transfer_statement_description_constructor_params<'a, P: IPairingAdapter>(
    pairing: &'a P,
    _overrides: TransferStatementDescriptionConstructorParamsOverrides,
) -> TransferStatementDescriptionConstructorParams<'a, P> {
    TransferStatementDescriptionConstructorParams { pairing }
}

pub fn build_transfer_statement_description<'a, P: IPairingAdapter, F: IChainForms>(
    pairing: &'a P,
    overrides: TransferStatementDescriptionConstructorParamsOverrides,
) -> TransferStatementDescription<'a, P, F> {
    let Ok(description) = TransferStatementDescription::try_new(
        build_transfer_statement_description_constructor_params(pairing, overrides),
    );
    description
}

fn form_from_seed<T: ICanonicalField>(seed: u8) -> T {
    T::from_field(
        build_from_field_params(FromFieldParamsOverrides::default()),
        seeded_field(T::KIND, seed),
    )
    .expect("a constant of the declared kind is admitted")
    .value
}

fn seeded_field(kind: CanonicalFieldKind, seed: u8) -> CanonicalFieldValue {
    match kind {
        CanonicalFieldKind::FixedBytes20 => CanonicalFieldValue::FixedBytes20([seed; 20]),
        CanonicalFieldKind::FixedBytes32 => CanonicalFieldValue::FixedBytes32([seed; 32]),
        CanonicalFieldKind::Unsigned16 => CanonicalFieldValue::Unsigned16(u16::from(seed)),
        CanonicalFieldKind::Unsigned32 => CanonicalFieldValue::Unsigned32(u32::from(seed)),
        CanonicalFieldKind::Unsigned64 => CanonicalFieldValue::Unsigned64(u64::from(seed)),
        CanonicalFieldKind::Unsigned256 => CanonicalFieldValue::Unsigned256([seed; 32]),
        CanonicalFieldKind::Bytes => CanonicalFieldValue::Bytes(vec![seed]),
        CanonicalFieldKind::Text => CanonicalFieldValue::Text(seed.to_string()),
    }
}

fn sampled_scalar<P: IPairingAdapter>(fill: u8) -> P::Scalar {
    P::Scalar::sample_from_uniform_bytes(
        SampleUniformScalarParams,
        SampleUniformScalarPayload {
            uniform: build_secret(
                vec![fill; <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH],
                SecretConstructorParamsOverrides,
            ),
        },
    )
    .expect("sampling from uniform bytes of the declared length always succeeds")
    .scalar
    .expose()
    .clone()
}

fn g1_mul<P: IPairingAdapter>(pairing: &P, fill: u8) -> P::G1 {
    let Ok(generator) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
    let Ok(product) = pairing.mul_g1(
        MulG1Params,
        MulG1Payload {
            point: generator.point,
            scalar: sampled_scalar::<P>(fill),
        },
    );
    product.product
}

fn g2_mul<P: IPairingAdapter>(pairing: &P, fill: u8) -> P::G2 {
    let Ok(generator) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
    let Ok(product) = pairing.mul_g2(
        MulG2Params,
        MulG2Payload {
            point: generator.point,
            scalar: sampled_scalar::<P>(fill),
        },
    );
    product.product
}

fn render_form<T: ICanonicalField>(form: &T) -> CanonicalFieldValue {
    let Ok(rendered) = T::to_field(build_to_field_params(ToFieldParamsOverrides), form);
    rendered.field
}

fn encode_g1_field<P: IPairingAdapter>(pairing: &P, point: &P::G1) -> CanonicalFieldValue {
    let Ok(encoded) = pairing.encode_g1(
        EncodeG1Params,
        build_encode_g1_payload(
            pairing,
            EncodeG1PayloadOverrides {
                point: Some(point.clone()),
            },
        ),
    );
    CanonicalFieldValue::Bytes(encoded.bytes.as_ref().to_vec())
}

fn encode_g2_field<P: IPairingAdapter>(pairing: &P, point: &P::G2) -> CanonicalFieldValue {
    let Ok(encoded) = pairing.encode_g2(
        EncodeG2Params,
        build_encode_g2_payload(
            pairing,
            EncodeG2PayloadOverrides {
                point: Some(point.clone()),
            },
        ),
    );
    CanonicalFieldValue::Bytes(encoded.bytes.as_ref().to_vec())
}
