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

use encoding::{CreateEncodingDeps, CreateEncodingParams, CreateEncodingPayload, create_encoding};
use hash_to_scalar::{
    CreateHashToScalarDeps, CreateHashToScalarParams, CreateHashToScalarPayload,
    create_hash_to_scalar,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, IPairingArithmetic, IPairingConsumer,
    IPairingReference,
};

use crate::run::provides::{EncodingConsumer, EncodingConsumerConstructorParams};
use interface::{
    PairingConsumer, PairingConsumerConstructorParams, PairingConsumerErrorReturn,
    PairingConsumerReturn, PairingConsumerTryNewReturn,
};

impl<'a> PairingConsumer<'a> {
    pub fn try_new(
        params: PairingConsumerConstructorParams<'a>,
    ) -> PairingConsumerTryNewReturn<'a> {
        Ok(PairingConsumer {
            random: params.random,
            statements: params.statements,
            identities: params.identities,
            hash_to_scalar: params.hash_to_scalar,
            encoding: params.encoding,
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

impl<'a> IPairingConsumer for PairingConsumer<'a> {
    type Output = PairingConsumerReturn;

    fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let adapter = match create_hash_to_scalar::<P::Scalar>(
            &CreateHashToScalarDeps,
            CreateHashToScalarParams {
                concrete: self.hash_to_scalar.concrete,
                identifier: self.hash_to_scalar.identifier,
            },
            CreateHashToScalarPayload,
        ) {
            Err(error) => return Err(PairingConsumerErrorReturn::CreateHashToScalar(error)),
            Ok(success) => success.adapter,
        };
        let Ok(consumer) = EncodingConsumer::try_new(EncodingConsumerConstructorParams {
            pairing: &payload.adapter,
            hash_to_scalar: &*adapter,
            random: self.random,
            statements: self.statements,
            identities: self.identities,
            chain_forms: self.chain_forms,
            kem: self.kem,
            key_agreement: self.key_agreement,
            delivery_proof: self.delivery_proof,
            generate: self.generate.clone(),
            kinds: self.kinds.clone(),
            verifier: self.verifier.clone(),
            statement_version: self.statement_version,
            iterations: self.iterations,
        });
        let output = match create_encoding(
            &CreateEncodingDeps { consumer },
            CreateEncodingParams {
                concrete: self.encoding.concrete,
                identifier: self.encoding.identifier,
            },
            CreateEncodingPayload,
        ) {
            Err(error) => return Err(PairingConsumerErrorReturn::CreateEncoding(error)),
            Ok(success) => success.output,
        };
        match output {
            Err(error) => Err(PairingConsumerErrorReturn::Encoding(error)),
            Ok(success) => Ok(success),
        }
    }
}
