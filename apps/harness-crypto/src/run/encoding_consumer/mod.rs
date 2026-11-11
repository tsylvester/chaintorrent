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

use chain::{
    CreateChainFormsDeps, CreateChainFormsParams, CreateChainFormsPayload, create_chain_forms,
};
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, IDecoderAdapter, IEncoderAdapter,
    IEncodingConsumer,
};
use pairing::{IPairingArithmetic, IPairingReference};

use crate::run::provides::{ChainFormsConsumer, ChainFormsConsumerConstructorParams};
use interface::{
    EncodingConsumer, EncodingConsumerConstructorParams, EncodingConsumerErrorReturn,
    EncodingConsumerReturn, EncodingConsumerTryNewReturn,
};

impl<'a, P> EncodingConsumer<'a, P>
where
    P: IPairingArithmetic + IPairingReference,
{
    pub fn try_new(
        params: EncodingConsumerConstructorParams<'a, P>,
    ) -> EncodingConsumerTryNewReturn<'a, P> {
        Ok(EncodingConsumer {
            pairing: params.pairing,
            hash_to_scalar: params.hash_to_scalar,
            random: params.random,
            statements: params.statements,
            identities: params.identities,
            chain_forms: params.chain_forms,
            kem: params.kem,
            key_agreement: params.key_agreement,
            delivery_proof: params.delivery_proof,
            generate: params.generate,
            kinds: params.kinds,
            verifier: params.verifier,
            statement_version: params.statement_version,
            iterations: params.iterations,
        })
    }
}

impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
where
    P: IPairingArithmetic + IPairingReference,
{
    type Output = EncodingConsumerReturn;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        let Ok(consumer) = ChainFormsConsumer::try_new(ChainFormsConsumerConstructorParams {
            pairing: self.pairing,
            hash_to_scalar: self.hash_to_scalar,
            encoder: &payload.adapter,
            random: self.random,
            statements: self.statements,
            identities: self.identities,
            kem: self.kem,
            key_agreement: self.key_agreement,
            delivery_proof: self.delivery_proof,
            generate: self.generate.clone(),
            kinds: self.kinds.clone(),
            verifier: self.verifier.clone(),
            statement_version: self.statement_version,
            iterations: self.iterations,
        });
        let output = match create_chain_forms(
            &CreateChainFormsDeps {
                consumer,
                encoder: &payload.adapter,
            },
            CreateChainFormsParams {
                concrete: self.chain_forms.concrete,
                identifier: self.chain_forms.identifier,
            },
            CreateChainFormsPayload,
        ) {
            Err(error) => return Err(EncodingConsumerErrorReturn::CreateChainForms(error)),
            Ok(success) => success.output,
        };
        match output {
            Err(error) => Err(EncodingConsumerErrorReturn::ChainForms(error)),
            Ok(success) => Ok(success),
        }
    }
}
