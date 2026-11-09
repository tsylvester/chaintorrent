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

use chain::{ConsumeChainFormsParams, ConsumeChainFormsPayload, IChainForms, IChainFormsConsumer};
use encoding::IEncoderAdapter;
use kem::{CreateKemDeps, CreateKemParams, CreateKemPayload, create_kem};
use pairing::{IPairingArithmetic, IPairingReference};
use proof::{DeliveryContext, MintPurpose, TransferPurpose};

use crate::config::provides::AdmittedStatements;
use crate::run::provides::{
    AdmitContextsDeps, AdmitContextsParams, AdmitContextsPayload, KemConsumer,
    KemConsumerConstructorParams, admit_contexts,
};
use interface::{
    ChainFormsConsumer, ChainFormsConsumerConstructorParams, ChainFormsConsumerErrorReturn,
    ChainFormsConsumerReturn, ChainFormsConsumerTryNewReturn,
};

pub(super) fn admit_statement_contexts<F: IChainForms>(
    statements: &AdmittedStatements,
) -> Result<
    (
        DeliveryContext<F, MintPurpose>,
        DeliveryContext<F, TransferPurpose>,
    ),
    ChainFormsConsumerErrorReturn,
> {
    let mint = match admit_contexts::<F, MintPurpose>(
        &AdmitContextsDeps,
        AdmitContextsParams,
        AdmitContextsPayload {
            context: &statements.mint,
            purpose: statements.mint_purpose,
        },
    ) {
        Err(error) => return Err(ChainFormsConsumerErrorReturn::AdmitMint(error)),
        Ok(success) => success.context,
    };
    let transfer = match admit_contexts::<F, TransferPurpose>(
        &AdmitContextsDeps,
        AdmitContextsParams,
        AdmitContextsPayload {
            context: &statements.transfer,
            purpose: statements.transfer_purpose,
        },
    ) {
        Err(error) => return Err(ChainFormsConsumerErrorReturn::AdmitTransfer(error)),
        Ok(success) => success.context,
    };
    Ok((mint, transfer))
}

impl<'a, P, E> ChainFormsConsumer<'a, P, E>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
{
    pub fn try_new(
        params: ChainFormsConsumerConstructorParams<'a, P, E>,
    ) -> ChainFormsConsumerTryNewReturn<'a, P, E> {
        Ok(ChainFormsConsumer {
            pairing: params.pairing,
            hash_to_scalar: params.hash_to_scalar,
            encoder: params.encoder,
            random: params.random,
            statements: params.statements,
            identities: params.identities,
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

impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
where
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter,
{
    type Output = ChainFormsConsumerReturn;

    fn consume_chain_forms<F: IChainForms>(
        &self,
        _params: ConsumeChainFormsParams,
        _payload: ConsumeChainFormsPayload<F>,
    ) -> Self::Output {
        let (mint, transfer) = admit_statement_contexts::<F>(self.statements)?;
        let Ok(consumer) = KemConsumer::try_new(KemConsumerConstructorParams {
            pairing: self.pairing,
            hash_to_scalar: self.hash_to_scalar,
            encoder: self.encoder,
            random: self.random,
            mint,
            transfer,
            identities: self.identities,
            key_agreement: self.key_agreement,
            delivery_proof: self.delivery_proof,
            generate: self.generate.clone(),
            kinds: self.kinds.clone(),
            verifier: self.verifier.clone(),
            statement_version: self.statement_version,
            iterations: self.iterations,
        });
        let output = match create_kem(
            &CreateKemDeps {
                pairing: self.pairing,
                hash_to_scalar: self.hash_to_scalar,
                consumer,
            },
            CreateKemParams {
                concrete: self.kem.concrete,
                identifier: self.kem.identifier,
                scope: self.kem.scope,
            },
            CreateKemPayload,
        ) {
            Err(error) => return Err(ChainFormsConsumerErrorReturn::CreateKem(error)),
            Ok(success) => success.output,
        };
        match output {
            Err(error) => Err(ChainFormsConsumerErrorReturn::Kem(error)),
            Ok(success) => Ok(success),
        }
    }
}
