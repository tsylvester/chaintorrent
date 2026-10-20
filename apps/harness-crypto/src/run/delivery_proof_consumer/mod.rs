#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use chain::IChainForms;
use encoding::IEncoderAdapter;
use envelope::IKeyAgreementAdapter;
use kem::ICredentialKemAdapter;
use pairing::{IPairingArithmetic, IPairingReference};
use proof::{
    ConsumeDeliveryProofParams, ConsumeDeliveryProofPayload, IDeliveryProofAdapter,
    IDeliveryProofConsumer,
};

use super::generate_consumer::provides::{GenerateConsumer, GenerateConsumerConstructorParams};
use crate::generate::provides::{
    CreateGenerateDeps, CreateGenerateParams, CreateGeneratePayload, CreateGenerateReturn,
    EmitReturn, create_generate,
};
use interface::{
    DeliveryProofConsumer, DeliveryProofConsumerConstructorParams,
    DeliveryProofConsumerTryNewReturn,
};

impl<'a, P, E, K, A> DeliveryProofConsumer<'a, P, E, K, A>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
{
    pub fn try_new(
        params: DeliveryProofConsumerConstructorParams<'a, P, E, K, A>,
    ) -> DeliveryProofConsumerTryNewReturn<'a, P, E, K, A> {
        Ok(DeliveryProofConsumer {
            pairing: params.pairing,
            hash_to_scalar: params.hash_to_scalar,
            encoder: params.encoder,
            random: params.random,
            kem: params.kem,
            key_agreement: params.key_agreement,
            generate: params.generate,
            kinds: params.kinds,
            scope: params.scope,
            statement_version: params.statement_version,
            statements: params.statements,
        })
    }
}

impl<P, E, K, A, F: IChainForms> IDeliveryProofConsumer<P, F>
    for DeliveryProofConsumer<'_, P, E, K, A>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
{
    type Output = CreateGenerateReturn<EmitReturn>;

    fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
        &self,
        _params: ConsumeDeliveryProofParams,
        payload: ConsumeDeliveryProofPayload<D>,
    ) -> Self::Output {
        let Ok(consumer) = GenerateConsumer::try_new(GenerateConsumerConstructorParams {
            statements: self.statements,
        });
        create_generate(
            &CreateGenerateDeps {
                pairing: self.pairing,
                hash_to_scalar: self.hash_to_scalar,
                encoder: self.encoder,
                random: self.random,
                kem: self.kem,
                key_agreement: self.key_agreement,
                delivery_proof: &payload.adapter,
                consumer,
            },
            CreateGenerateParams {
                concrete: self.generate.clone(),
                kinds: self.kinds.to_vec(),
                scope: self.scope,
                statement_version: self.statement_version,
            },
            CreateGeneratePayload,
        )
    }
}
