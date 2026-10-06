#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{DELIVERY_PURPOSE_MINT, MintFirstMessages, MintPurpose, MintStatement};
use chain::IChainForms;
use domain::{
    AssetIdentityHash, AssetIdentityHashConstructorParamsOverrides, ParameterSetIdentifier,
    ParameterSetIdentifierConstructorParamsOverrides, SuiteIdentifier,
    SuiteIdentifierConstructorParamsOverrides, build_asset_identity_hash,
    build_parameter_set_identifier, build_suite_identifier,
};
use envelope::{
    EnvelopeComponents, PublicKeysComponents, build_envelope_components,
    build_public_keys_components,
};
use pairing::{
    AddG1Params, AddG1Payload, AddG2Params, AddG2Payload, G1GeneratorParams, G1GeneratorPayload,
    G2GeneratorParams, G2GeneratorPayload, IPairingAdapter,
};

pub struct MintFirstMessagesOverrides<G1, G2> {
    pub hpub: Option<G2>,
    pub c1: Option<G1>,
    pub c2: Option<G1>,
    pub d1: Option<G2>,
    pub d2: Option<G2>,
}

impl<G1, G2> Default for MintFirstMessagesOverrides<G1, G2> {
    fn default() -> Self {
        Self {
            hpub: None,
            c1: None,
            c2: None,
            d1: None,
            d2: None,
        }
    }
}

pub fn build_mint_first_messages<P: IPairingAdapter>(
    pairing: &P,
    overrides: MintFirstMessagesOverrides<P::G1, P::G2>,
) -> MintFirstMessages<P::G1, P::G2> {
    let Ok(g1) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
    let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
    let Ok(g1g1) = pairing.add_g1(
        AddG1Params,
        AddG1Payload {
            left: g1.point.clone(),
            right: g1.point.clone(),
        },
    );
    let Ok(g1g1g1) = pairing.add_g1(
        AddG1Params,
        AddG1Payload {
            left: g1g1.sum.clone(),
            right: g1.point.clone(),
        },
    );
    let Ok(g2g2) = pairing.add_g2(
        AddG2Params,
        AddG2Payload {
            left: g2.point.clone(),
            right: g2.point.clone(),
        },
    );
    let Ok(g2g2g2) = pairing.add_g2(
        AddG2Params,
        AddG2Payload {
            left: g2g2.sum.clone(),
            right: g2.point.clone(),
        },
    );
    MintFirstMessages {
        hpub: overrides.hpub.unwrap_or(g2g2.sum),
        c1: overrides.c1.unwrap_or(g1g1.sum),
        c2: overrides.c2.unwrap_or(g1g1g1.sum),
        d1: overrides.d1.unwrap_or(g2.point.clone()),
        d2: overrides.d2.unwrap_or(g2g2g2.sum),
    }
}

pub struct MintStatementOverrides<P: IPairingAdapter, F: IChainForms> {
    pub suite_identifier: Option<SuiteIdentifier>,
    pub chain: Option<F::ChainIdentifier>,
    pub entitlement_contract: Option<F::Identity>,
    pub asset_identity_hash: Option<AssetIdentityHash>,
    pub parameter_set_identifier: Option<ParameterSetIdentifier>,
    pub source_entitlement: Option<F::Entitlement>,
    pub target_entitlement: Option<F::Entitlement>,
    pub old_interval: Option<F::Interval>,
    pub new_interval: Option<F::Interval>,
    pub purpose: Option<MintPurpose>,
    pub seller: Option<F::Identity>,
    pub buyer: Option<F::Identity>,
    pub buyer_keys: Option<PublicKeysComponents<P::G1, P::G2>>,
    pub envelope: Option<EnvelopeComponents<P::G1, P::G2>>,
    pub expiry: Option<u64>,
    pub first_messages: Option<MintFirstMessages<P::G1, P::G2>>,
}

impl<P: IPairingAdapter, F: IChainForms> Default for MintStatementOverrides<P, F> {
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
            buyer_keys: None,
            envelope: None,
            expiry: None,
            first_messages: None,
        }
    }
}

pub fn build_mint_statement<P: IPairingAdapter, F: IChainForms>(
    pairing: &P,
    overrides: MintStatementOverrides<P, F>,
) -> MintStatement<P, F>
where
    F::ChainIdentifier: Default,
    F::Entitlement: Default,
    F::Identity: Default,
    F::Interval: Default,
{
    MintStatement {
        suite_identifier: overrides.suite_identifier.unwrap_or_else(|| {
            build_suite_identifier(SuiteIdentifierConstructorParamsOverrides::default())
        }),
        chain: overrides.chain.unwrap_or_default(),
        entitlement_contract: overrides.entitlement_contract.unwrap_or_default(),
        asset_identity_hash: overrides.asset_identity_hash.unwrap_or_else(|| {
            build_asset_identity_hash(AssetIdentityHashConstructorParamsOverrides::default())
        }),
        parameter_set_identifier: overrides.parameter_set_identifier.unwrap_or_else(|| {
            build_parameter_set_identifier(
                ParameterSetIdentifierConstructorParamsOverrides::default(),
            )
        }),
        source_entitlement: overrides.source_entitlement.unwrap_or_default(),
        target_entitlement: overrides.target_entitlement.unwrap_or_default(),
        old_interval: overrides.old_interval.unwrap_or_default(),
        new_interval: overrides.new_interval.unwrap_or_default(),
        purpose: overrides.purpose.unwrap_or(DELIVERY_PURPOSE_MINT),
        seller: overrides.seller.unwrap_or_default(),
        buyer: overrides.buyer.unwrap_or_default(),
        buyer_keys: overrides
            .buyer_keys
            .unwrap_or_else(|| build_public_keys_components(pairing, Default::default())),
        envelope: overrides
            .envelope
            .unwrap_or_else(|| build_envelope_components(pairing, Default::default())),
        expiry: overrides.expiry.unwrap_or(1_700_000_000),
        first_messages: overrides
            .first_messages
            .unwrap_or_else(|| build_mint_first_messages(pairing, Default::default())),
    }
}
