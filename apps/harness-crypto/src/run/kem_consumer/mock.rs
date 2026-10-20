#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use chain::IChainForms;
use encoding::IEncoderAdapter;
use hash_to_scalar::IHashToScalarAdapter;
use pairing::{IPairingArithmetic, IPairingReference};
use random::IRandomSourceAdapter;

use super::interface::{KemConsumer, KemConsumerConstructorParams};
use crate::config::provides::{
    DeliveryProofSelection, HarnessStatements, KeyAgreementSelection,
    build_delivery_proof_selection, build_key_agreement_selection,
};
use crate::generate::provides::{GenerateConcrete, VectorKindConfiguration};

#[derive(Default)]
pub struct KemConsumerConstructorParamsOverrides {
    pub key_agreement: Option<KeyAgreementSelection>,
    pub delivery_proof: Option<DeliveryProofSelection>,
}

pub fn build_kem_consumer_constructor_params<'a, P, E>(
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    generate: &'a GenerateConcrete,
    kinds: &'a [VectorKindConfiguration],
    statements: &'a HarnessStatements,
    overrides: KemConsumerConstructorParamsOverrides,
) -> KemConsumerConstructorParams<'a, P, E>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
{
    KemConsumerConstructorParams {
        pairing,
        hash_to_scalar,
        encoder,
        random,
        generate,
        kinds,
        key_agreement: overrides
            .key_agreement
            .unwrap_or_else(|| build_key_agreement_selection(Default::default())),
        delivery_proof: overrides
            .delivery_proof
            .unwrap_or_else(|| build_delivery_proof_selection(Default::default())),
        statements,
    }
}

pub fn build_kem_consumer<'a, P, E, F>(
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    generate: &'a GenerateConcrete,
    kinds: &'a [VectorKindConfiguration],
    statements: &'a HarnessStatements,
    overrides: KemConsumerConstructorParamsOverrides,
) -> KemConsumer<'a, P, E, F>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    F: IChainForms,
{
    let Ok(consumer) = KemConsumer::try_new(build_kem_consumer_constructor_params(
        pairing,
        hash_to_scalar,
        encoder,
        random,
        generate,
        kinds,
        statements,
        overrides,
    ));
    consumer
}
