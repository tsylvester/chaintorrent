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
use envelope::IKeyAgreementAdapter;
use kem::ICredentialKemAdapter;
use pairing::{IPairingArithmetic, IPairingReference};
use proof::{
    ConsumeDeliveryProofParams, ConsumeDeliveryProofPayload, IDeliveryProofAdapter,
    IDeliveryProofConsumer,
};

use crate::generate::provides::{
    CreateGenerateDeps, CreateGenerateParams, CreateGeneratePayload, create_generate,
};
use crate::run::provides::{
    GenerateConsumer, GenerateConsumerConstructorParams, VerifierConsumer,
    VerifierConsumerConstructorParams,
};
use crate::verifier::provides::{
    CreateVerifierDeps, CreateVerifierParams, CreateVerifierPayload, create_verifier,
};
use interface::{
    DeliveryProofConsumer, DeliveryProofConsumerConstructorParams,
    DeliveryProofConsumerErrorReturn, DeliveryProofConsumerReturn,
    DeliveryProofConsumerSuccessReturn, DeliveryProofConsumerTryNewReturn,
};

impl<'a, P, E, K, A, F> DeliveryProofConsumer<'a, P, E, K, A, F>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    F: IChainForms,
{
    pub fn try_new(
        params: DeliveryProofConsumerConstructorParams<'a, P, E, K, A, F>,
    ) -> DeliveryProofConsumerTryNewReturn<'a, P, E, K, A, F> {
        Ok(DeliveryProofConsumer {
            pairing: params.pairing,
            hash_to_scalar: params.hash_to_scalar,
            encoder: params.encoder,
            random: params.random,
            kem: params.kem,
            key_agreement: params.key_agreement,
            mint: params.mint,
            transfer: params.transfer,
            identities: params.identities,
            proof_identity: params.proof_identity,
            generate: params.generate,
            kinds: params.kinds,
            verifier: params.verifier,
            statement_version: params.statement_version,
            iterations: params.iterations,
        })
    }
}

impl<'a, P, E, K, A, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'a, P, E, K, A, F>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    F: IChainForms,
{
    type Output = DeliveryProofConsumerReturn;

    fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
        &self,
        _params: ConsumeDeliveryProofParams,
        payload: ConsumeDeliveryProofPayload<D>,
    ) -> Self::Output {
        let Ok(generate_consumer) = GenerateConsumer::try_new(GenerateConsumerConstructorParams {
            mint: self.mint.clone(),
            transfer: self.transfer.clone(),
            identities: self.identities.clone(),
            proof_identity: self.proof_identity,
        });
        let generated = match create_generate(
            &CreateGenerateDeps {
                pairing: self.pairing,
                hash_to_scalar: self.hash_to_scalar,
                encoder: self.encoder,
                random: self.random,
                kem: self.kem,
                key_agreement: self.key_agreement,
                delivery_proof: &payload.adapter,
                consumer: generate_consumer,
            },
            CreateGenerateParams {
                concrete: self.generate.clone(),
                kinds: self.kinds.clone(),
                statement_version: self.statement_version,
            },
            CreateGeneratePayload,
        ) {
            Err(error) => return Err(DeliveryProofConsumerErrorReturn::CreateGenerate(error)),
            Ok(success) => success.output,
        };
        let files = match generated {
            Err(error) => return Err(DeliveryProofConsumerErrorReturn::Emit(error)),
            Ok(success) => success.files,
        };
        let Ok(verifier_consumer) = VerifierConsumer::try_new(VerifierConsumerConstructorParams {
            pairing: self.pairing,
            kem: self.kem,
            key_agreement: self.key_agreement,
            delivery_proof: &payload.adapter,
            random: self.random,
            mint: self.mint.clone(),
            transfer: self.transfer.clone(),
            identity: self.proof_identity,
            iterations: self.iterations,
        });
        let measured = match create_verifier(
            &CreateVerifierDeps {
                pairing: self.pairing,
                delivery_proof: &payload.adapter,
                consumer: verifier_consumer,
            },
            CreateVerifierParams {
                concrete: self.verifier.clone(),
                statement_version: self.statement_version,
            },
            CreateVerifierPayload,
        ) {
            Err(error) => return Err(DeliveryProofConsumerErrorReturn::CreateVerifier(error)),
            Ok(success) => success.output,
        };
        let measurement = match measured {
            Err(error) => return Err(DeliveryProofConsumerErrorReturn::Measure(error)),
            Ok(success) => success.measurement,
        };
        Ok(DeliveryProofConsumerSuccessReturn { files, measurement })
    }
}
