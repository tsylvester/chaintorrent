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
use pairing::IPairingArithmetic;

use crate::schnorr_fs::provides::{
    SchnorrFsDeliveryProof, SchnorrFsDeliveryProofConstructorParams,
};

#[cfg(any(test, feature = "mocks"))]
use self::interface::IDeliveryProofAdapter;
use self::interface::{
    ConsumeDeliveryProofParams, ConsumeDeliveryProofPayload, CreateDeliveryProofDeps,
    CreateDeliveryProofErrorReturn, CreateDeliveryProofParams, CreateDeliveryProofPayload,
    CreateDeliveryProofReturn, CreateDeliveryProofSuccessReturn, DeliveryProofConcrete,
    IDeliveryProofConsumer,
};
#[cfg(any(test, feature = "mocks"))]
use self::mock::{MockIDeliveryProofAdapter, MockIDeliveryProofAdapterConstructorParams};

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
) -> CreateDeliveryProofReturn<C::Output> {
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
        #[cfg(any(test, feature = "mocks"))]
        DeliveryProofConcrete::Mock(failure_mode) => {
            if !MockIDeliveryProofAdapter::<'a, P, F>::DECLARATION
                .algebras
                .contains(&params.algebra)
            {
                return Err(CreateDeliveryProofErrorReturn::UnsupportedEnvelopeAlgebra);
            }
            if !MockIDeliveryProofAdapter::<'a, P, F>::DECLARATION
                .verifier_forms
                .contains(&params.verifier_form)
            {
                return Err(CreateDeliveryProofErrorReturn::UnsupportedVerifierForm);
            }
            if !MockIDeliveryProofAdapter::<'a, P, F>::DECLARATION
                .statement_versions
                .contains(&params.statement_version)
            {
                return Err(CreateDeliveryProofErrorReturn::UnsupportedStatementVersion);
            }
            let Ok(adapter) =
                MockIDeliveryProofAdapter::try_new(MockIDeliveryProofAdapterConstructorParams {
                    pairing: deps.pairing,
                    verifier_form: params.verifier_form,
                    failure_mode,
                });
            let output = deps.consumer.consume_delivery_proof(
                ConsumeDeliveryProofParams,
                ConsumeDeliveryProofPayload { adapter },
            );
            Ok(CreateDeliveryProofSuccessReturn { output })
        }
    }
}
