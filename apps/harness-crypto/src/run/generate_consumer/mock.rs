#![allow(dead_code, clippy::too_many_arguments)]

use chain::IChainForms;
use proof::{DeliveryContext, MintPurpose, TransferPurpose};

use super::interface::{GenerateConsumer, GenerateConsumerConstructorParams};
use crate::delivery::provides::AdmittedKemIdentity;

#[derive(Default)]
pub struct GenerateConsumerConstructorParamsOverrides;

pub fn build_generate_consumer_constructor_params<'a, F: IChainForms>(
    mint: DeliveryContext<F, MintPurpose>,
    transfer: DeliveryContext<F, TransferPurpose>,
    identities: Vec<AdmittedKemIdentity<'a>>,
    proof_identity: AdmittedKemIdentity<'a>,
    _overrides: GenerateConsumerConstructorParamsOverrides,
) -> GenerateConsumerConstructorParams<'a, F> {
    GenerateConsumerConstructorParams {
        mint,
        transfer,
        identities,
        proof_identity,
    }
}

pub fn build_generate_consumer<'a, F: IChainForms>(
    mint: DeliveryContext<F, MintPurpose>,
    transfer: DeliveryContext<F, TransferPurpose>,
    identities: Vec<AdmittedKemIdentity<'a>>,
    proof_identity: AdmittedKemIdentity<'a>,
    overrides: GenerateConsumerConstructorParamsOverrides,
) -> GenerateConsumer<'a, F> {
    let Ok(consumer) = GenerateConsumer::try_new(build_generate_consumer_constructor_params(
        mint,
        transfer,
        identities,
        proof_identity,
        overrides,
    ));
    consumer
}
