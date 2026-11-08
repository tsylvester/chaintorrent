#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test_basic;
#[cfg(test)]
mod test_error;

use chain::IChainForms;
use encoding::IEncoderAdapter;
use envelope::{
    ConsumeKeyAgreementParams, ConsumeKeyAgreementPayload, IKeyAgreementAdapter,
    IKeyAgreementConsumer,
};
use kem::ICredentialKemAdapter;
use pairing::{IPairingArithmetic, IPairingReference};
use proof::{
    CreateDeliveryProofDeps, CreateDeliveryProofParams, CreateDeliveryProofPayload,
    create_delivery_proof,
};

use crate::run::provides::{DeliveryProofConsumer, DeliveryProofConsumerConstructorParams};
use interface::{
    KeyAgreementConsumer, KeyAgreementConsumerConstructorParams, KeyAgreementConsumerErrorReturn,
    KeyAgreementConsumerReturn, KeyAgreementConsumerTryNewReturn,
};

impl<'a, P, E, K, F> KeyAgreementConsumer<'a, P, E, K, F>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    F: IChainForms,
{
    pub fn try_new(
        params: KeyAgreementConsumerConstructorParams<'a, P, E, K, F>,
    ) -> KeyAgreementConsumerTryNewReturn<'a, P, E, K, F> {
        Ok(KeyAgreementConsumer {
            pairing: params.pairing,
            hash_to_scalar: params.hash_to_scalar,
            encoder: params.encoder,
            random: params.random,
            kem: params.kem,
            mint: params.mint,
            transfer: params.transfer,
            identities: params.identities,
            proof_identity: params.proof_identity,
            delivery_proof: params.delivery_proof,
            generate: params.generate,
            kinds: params.kinds,
            verifier: params.verifier,
            statement_version: params.statement_version,
            iterations: params.iterations,
        })
    }
}

impl<'a, P, E, K, F> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P, E, K, F>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    F: IChainForms,
{
    type Output = KeyAgreementConsumerReturn;

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
            mint: self.mint.clone(),
            transfer: self.transfer.clone(),
            identities: self.identities.clone(),
            proof_identity: self.proof_identity,
            generate: self.generate.clone(),
            kinds: self.kinds.clone(),
            verifier: self.verifier.clone(),
            statement_version: self.statement_version,
            iterations: self.iterations,
        });
        let output = match create_delivery_proof(
            &CreateDeliveryProofDeps {
                pairing: self.pairing,
                hash_to_scalar: self.hash_to_scalar,
                encoder: self.encoder,
                random: self.random,
                consumer,
            },
            CreateDeliveryProofParams {
                concrete: self.delivery_proof,
                algebra: A::DECLARATION.algebra,
                statement_version: self.statement_version,
            },
            CreateDeliveryProofPayload,
        ) {
            Err(error) => return Err(KeyAgreementConsumerErrorReturn::CreateDeliveryProof(error)),
            Ok(success) => success.output,
        };
        match output {
            Err(error) => Err(KeyAgreementConsumerErrorReturn::DeliveryProof(error)),
            Ok(success) => Ok(success),
        }
    }
}
