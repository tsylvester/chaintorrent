#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{TransferFirstMessages, TransferStatement};
use crate::mint_statement::provides::{DELIVERY_PURPOSE_TRANSFER, TransferPurpose};
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
    TransferFirstMessages {
        pk1: overrides.pk1.unwrap_or(g1.point.clone()),
        c1: overrides.c1.unwrap_or(g1g1.sum),
        c2: overrides.c2.unwrap_or(g1g1g1.sum),
        pk2: overrides.pk2.unwrap_or(g2.point.clone()),
        d1: overrides.d1.unwrap_or(g2g2.sum),
        d2: overrides.d2.unwrap_or(g2g2g2.sum),
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
) -> TransferStatement<P, F>
where
    F::ChainIdentifier: Default,
    F::Entitlement: Default,
    F::Identity: Default,
    F::Interval: Default,
{
    TransferStatement {
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
        purpose: overrides.purpose.unwrap_or(DELIVERY_PURPOSE_TRANSFER),
        seller: overrides.seller.unwrap_or_default(),
        buyer: overrides.buyer.unwrap_or_default(),
        seller_keys: overrides
            .seller_keys
            .unwrap_or_else(|| build_public_keys_components(pairing, Default::default())),
        buyer_keys: overrides
            .buyer_keys
            .unwrap_or_else(|| build_public_keys_components(pairing, Default::default())),
        old_envelope: overrides
            .old_envelope
            .unwrap_or_else(|| build_envelope_components(pairing, Default::default())),
        new_envelope: overrides
            .new_envelope
            .unwrap_or_else(|| build_envelope_components(pairing, Default::default())),
        expiry: overrides.expiry.unwrap_or(1_700_000_000),
        first_messages: overrides
            .first_messages
            .unwrap_or_else(|| build_transfer_first_messages(pairing, Default::default())),
    }
}
