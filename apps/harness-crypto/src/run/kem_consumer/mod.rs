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
    CreateKeyAgreementDeps, CreateKeyAgreementParams, CreateKeyAgreementPayload,
    CreateKeyAgreementReturn, create_key_agreement,
};
use kem::{ConsumeKemParams, ConsumeKemPayload, ICredentialKemAdapter, IKemConsumer};
use pairing::{IPairingArithmetic, IPairingReference};
use proof::CreateDeliveryProofReturn;

use super::key_agreement_consumer::provides::{
    KeyAgreementConsumer, KeyAgreementConsumerConstructorParams,
};
use crate::generate::provides::{CreateGenerateReturn, EmitReturn};
use interface::{KemConsumer, KemConsumerConstructorParams, KemConsumerTryNewReturn};

impl<'a, P, E, F> KemConsumer<'a, P, E, F>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    F: IChainForms,
{
    pub fn try_new(
        params: KemConsumerConstructorParams<'a, P, E>,
    ) -> KemConsumerTryNewReturn<'a, P, E, F> {
        Ok(KemConsumer {
            pairing: params.pairing,
            hash_to_scalar: params.hash_to_scalar,
            encoder: params.encoder,
            random: params.random,
            generate: params.generate,
            kinds: params.kinds,
            key_agreement: params.key_agreement,
            delivery_proof: params.delivery_proof,
            statements: params.statements,
            forms: PhantomData,
        })
    }
}

impl<P, E, F> IKemConsumer<P> for KemConsumer<'_, P, E, F>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    F: IChainForms,
{
    type Output =
        CreateKeyAgreementReturn<CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>>;

    fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
        &self,
        _params: ConsumeKemParams,
        payload: ConsumeKemPayload<K>,
    ) -> Self::Output {
        let Ok(consumer) =
            KeyAgreementConsumer::<P, E, K, F>::try_new(KeyAgreementConsumerConstructorParams {
                pairing: self.pairing,
                hash_to_scalar: self.hash_to_scalar,
                encoder: self.encoder,
                random: self.random,
                kem: &payload.adapter,
                generate: self.generate,
                kinds: self.kinds,
                scope: payload.scope(),
                delivery_proof: self.delivery_proof,
                statements: self.statements,
            });
        create_key_agreement(
            &CreateKeyAgreementDeps {
                pairing: self.pairing,
                hash_to_scalar: self.hash_to_scalar,
                encoder: self.encoder,
                random: self.random,
                consumer,
            },
            CreateKeyAgreementParams {
                concrete: self.key_agreement.concrete,
                identifier: self.key_agreement.identifier,
                algebra: self.key_agreement.algebra,
            },
            CreateKeyAgreementPayload,
        )
    }
}
