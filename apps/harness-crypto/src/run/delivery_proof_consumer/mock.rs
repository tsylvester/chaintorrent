#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use encoding::IEncoderAdapter;
use envelope::IKeyAgreementAdapter;
use hash_to_scalar::IHashToScalarAdapter;
use kem::{ICredentialKemAdapter, IdentityScope};
use pairing::{IPairingArithmetic, IPairingReference};
use proof::DELIVERY_STATEMENT_VERSION_ONE;
use random::IRandomSourceAdapter;

use super::interface::{DeliveryProofConsumer, DeliveryProofConsumerConstructorParams};
use crate::config::provides::HarnessStatements;
use crate::generate::provides::{GenerateConcrete, VectorKindConfiguration};

#[derive(Default)]
pub struct DeliveryProofConsumerConstructorParamsOverrides {
    pub scope: Option<IdentityScope>,
    pub statement_version: Option<u16>,
}

pub fn build_delivery_proof_consumer_constructor_params<
    'a,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
>(
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    kem: &'a K,
    key_agreement: &'a A,
    generate: &'a GenerateConcrete,
    kinds: &'a [VectorKindConfiguration],
    statements: &'a HarnessStatements,
    overrides: DeliveryProofConsumerConstructorParamsOverrides,
) -> DeliveryProofConsumerConstructorParams<'a, P, E, K, A> {
    DeliveryProofConsumerConstructorParams {
        pairing,
        hash_to_scalar,
        encoder,
        random,
        kem,
        key_agreement,
        generate,
        kinds,
        scope: overrides.scope.unwrap_or(IdentityScope::Entitlement),
        statement_version: overrides
            .statement_version
            .unwrap_or(DELIVERY_STATEMENT_VERSION_ONE),
        statements,
    }
}

pub fn build_delivery_proof_consumer<
    'a,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
>(
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    kem: &'a K,
    key_agreement: &'a A,
    generate: &'a GenerateConcrete,
    kinds: &'a [VectorKindConfiguration],
    statements: &'a HarnessStatements,
    overrides: DeliveryProofConsumerConstructorParamsOverrides,
) -> DeliveryProofConsumer<'a, P, E, K, A> {
    let Ok(consumer) =
        DeliveryProofConsumer::try_new(build_delivery_proof_consumer_constructor_params(
            pairing,
            hash_to_scalar,
            encoder,
            random,
            kem,
            key_agreement,
            generate,
            kinds,
            statements,
            overrides,
        ));
    consumer
}
