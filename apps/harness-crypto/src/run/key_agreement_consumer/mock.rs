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
use kem::{ICredentialKemAdapter, IdentityScope};
use pairing::{IPairingArithmetic, IPairingReference};
use random::IRandomSourceAdapter;

use super::interface::{KeyAgreementConsumer, KeyAgreementConsumerConstructorParams};
use crate::config::provides::{
    DeliveryProofSelection, HarnessStatements, build_delivery_proof_selection,
};
use crate::generate::provides::{GenerateConcrete, VectorKindConfiguration};

#[derive(Default)]
pub struct KeyAgreementConsumerConstructorParamsOverrides {
    pub scope: Option<IdentityScope>,
    pub delivery_proof: Option<DeliveryProofSelection>,
}

pub fn build_key_agreement_consumer_constructor_params<'a, P, E, K>(
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    kem: &'a K,
    generate: &'a GenerateConcrete,
    kinds: &'a [VectorKindConfiguration],
    statements: &'a HarnessStatements,
    overrides: KeyAgreementConsumerConstructorParamsOverrides,
) -> KeyAgreementConsumerConstructorParams<'a, P, E, K>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
{
    KeyAgreementConsumerConstructorParams {
        pairing,
        hash_to_scalar,
        encoder,
        random,
        kem,
        generate,
        kinds,
        scope: overrides.scope.unwrap_or(IdentityScope::Entitlement),
        delivery_proof: overrides
            .delivery_proof
            .unwrap_or_else(|| build_delivery_proof_selection(Default::default())),
        statements,
    }
}

pub fn build_key_agreement_consumer<'a, P, E, K, F>(
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    kem: &'a K,
    generate: &'a GenerateConcrete,
    kinds: &'a [VectorKindConfiguration],
    statements: &'a HarnessStatements,
    overrides: KeyAgreementConsumerConstructorParamsOverrides,
) -> KeyAgreementConsumer<'a, P, E, K, F>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    F: IChainForms,
{
    let Ok(consumer) =
        KeyAgreementConsumer::try_new(build_key_agreement_consumer_constructor_params(
            pairing,
            hash_to_scalar,
            encoder,
            random,
            kem,
            generate,
            kinds,
            statements,
            overrides,
        ));
    consumer
}
