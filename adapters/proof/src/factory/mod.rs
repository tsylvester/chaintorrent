mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use chain::IChainForms;
use encoding::IEncoderAdapter;
use pairing::IPairingArithmetic;

use crate::mint_statement::provides::MintStatementFromFieldsErrorReturn;
use crate::schnorr_fs::provides::{
    SchnorrFsDeliveryProof, SchnorrFsDeliveryProofConstructorParams,
};
use crate::transfer_statement::provides::TransferStatementFromFieldsErrorReturn;

use self::interface::{
    ConsumeDeliveryProofParams, ConsumeDeliveryProofPayload, CreateDeliveryProofDeps,
    CreateDeliveryProofErrorReturn, CreateDeliveryProofParams, CreateDeliveryProofPayload,
    CreateDeliveryProofReturn, CreateDeliveryProofSuccessReturn, DeliveryProofConcrete,
    IDeliveryProofConsumer,
};

pub fn create_delivery_proof<
    'a,
    P: IPairingArithmetic,
    E: IEncoderAdapter,
    F: IChainForms,
    C: IDeliveryProofConsumer<P, F>,
>(
    deps: &CreateDeliveryProofDeps<'a, P, E, C>,
    params: CreateDeliveryProofParams,
    _payload: CreateDeliveryProofPayload,
) -> CreateDeliveryProofReturn<C::Output>
where
    MintStatementFromFieldsErrorReturn<F>: core::fmt::Debug + PartialEq + Eq,
    TransferStatementFromFieldsErrorReturn<F>: core::fmt::Debug + PartialEq + Eq,
{
    match params.concrete {
        DeliveryProofConcrete::SchnorrFs => {
            if !SchnorrFsDeliveryProof::<'a, P, E, F>::DECLARATION
                .algebras
                .contains(&params.algebra)
            {
                return Err(CreateDeliveryProofErrorReturn::UnsupportedEnvelopeAlgebra);
            }
            if !SchnorrFsDeliveryProof::<'a, P, E, F>::DECLARATION
                .verifier_forms
                .contains(&params.verifier_form)
            {
                return Err(CreateDeliveryProofErrorReturn::UnsupportedVerifierForm);
            }
            if !SchnorrFsDeliveryProof::<'a, P, E, F>::DECLARATION
                .statement_versions
                .contains(&params.statement_version)
            {
                return Err(CreateDeliveryProofErrorReturn::UnsupportedStatementVersion);
            }
            let adapter = match SchnorrFsDeliveryProof::<'a, P, E, F>::try_new(
                SchnorrFsDeliveryProofConstructorParams {
                    pairing: deps.pairing,
                    hash_to_scalar: deps.hash_to_scalar,
                    encoder: deps.encoder,
                    random: deps.random,
                    verifier_form: params.verifier_form,
                },
            ) {
                Ok(adapter) => adapter,
                Err(error) => return Err(CreateDeliveryProofErrorReturn::SchnorrFs(error)),
            };
            let output = deps.consumer.consume_delivery_proof(
                ConsumeDeliveryProofParams,
                ConsumeDeliveryProofPayload { adapter },
            );
            Ok(CreateDeliveryProofSuccessReturn { output })
        }
    }
}
