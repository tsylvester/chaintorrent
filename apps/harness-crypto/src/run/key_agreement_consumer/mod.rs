#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use chain::IChainForms;
use core::marker::PhantomData;
use encoding::IEncoderAdapter;
use envelope::{
    ConsumeKeyAgreementParams, ConsumeKeyAgreementPayload, IKeyAgreementAdapter,
    IKeyAgreementConsumer,
};
use kem::ICredentialKemAdapter;
use pairing::{IPairingArithmetic, IPairingReference};
use proof::{
    CreateDeliveryProofDeps, CreateDeliveryProofParams, CreateDeliveryProofPayload,
    CreateDeliveryProofReturn, create_delivery_proof,
};

use super::delivery_proof_consumer::provides::{
    DeliveryProofConsumer, DeliveryProofConsumerConstructorParams,
};
use crate::generate::provides::{CreateGenerateReturn, EmitReturn};
use interface::{
    KeyAgreementConsumer, KeyAgreementConsumerConstructorParams, KeyAgreementConsumerTryNewReturn,
};

impl<'a, P, E, K, F> KeyAgreementConsumer<'a, P, E, K, F>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    F: IChainForms,
{
    pub fn try_new(
        params: KeyAgreementConsumerConstructorParams<'a, P, E, K>,
    ) -> KeyAgreementConsumerTryNewReturn<'a, P, E, K, F> {
        Ok(KeyAgreementConsumer {
            pairing: params.pairing,
            hash_to_scalar: params.hash_to_scalar,
            encoder: params.encoder,
            random: params.random,
            kem: params.kem,
            generate: params.generate,
            kinds: params.kinds,
            scope: params.scope,
            delivery_proof: params.delivery_proof,
            statements: params.statements,
            forms: PhantomData,
        })
    }
}

impl<P, E, K, F> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P, E, K, F>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    F: IChainForms,
{
    type Output = CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>;

    fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
        &self,
        _params: ConsumeKeyAgreementParams,
        payload: ConsumeKeyAgreementPayload<A>,
    ) -> Self::Output {
        let Ok(consumer) = DeliveryProofConsumer::try_new(DeliveryProofConsumerConstructorParams {
            pairing: self.pairing,
            hash_to_scalar: self.hash_to_scalar,
            encoder: self.encoder,
            random: self.random,
            kem: self.kem,
            key_agreement: &payload.adapter,
            generate: self.generate,
            kinds: self.kinds,
            scope: self.scope,
            statement_version: self.delivery_proof.statement_version,
            statements: self.statements,
        });
        create_delivery_proof::<P, E, F, _>(
            &CreateDeliveryProofDeps {
                pairing: self.pairing,
                hash_to_scalar: self.hash_to_scalar,
                encoder: self.encoder,
                random: self.random,
                consumer,
            },
            CreateDeliveryProofParams {
                concrete: self.delivery_proof.concrete,
                algebra: A::DECLARATION.algebra,
                verifier_form: P::DECLARATION.verifier_group_arithmetic,
                statement_version: self.delivery_proof.statement_version,
            },
            CreateDeliveryProofPayload,
        )
    }
}
