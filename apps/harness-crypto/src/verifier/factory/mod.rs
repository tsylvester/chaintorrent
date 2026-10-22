#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use pairing::IPairingAdapter;
use proof::IDeliveryProofAdapter;

use super::evm::provides::{EvmVerifier, EvmVerifierConstructorParams};

use self::interface::{
    ConsumeVerifierParams, ConsumeVerifierPayload, CreateVerifierDeps, CreateVerifierErrorReturn,
    CreateVerifierParams, CreateVerifierPayload, CreateVerifierReturn, CreateVerifierSuccessReturn,
    IVerifierAdapter, IVerifierConsumer, VerifierConcrete,
};
#[cfg(any(test, feature = "mocks"))]
use self::mock::{MockIVerifierAdapter, MockIVerifierAdapterConstructorParams};

pub fn create_verifier<
    'a,
    P: IPairingAdapter,
    D: IDeliveryProofAdapter<Pairing = P>,
    C: IVerifierConsumer<D>,
>(
    deps: &CreateVerifierDeps<'a, P, D, C>,
    params: CreateVerifierParams,
    _payload: CreateVerifierPayload,
) -> CreateVerifierReturn<C::Output> {
    let declaration = match &params.concrete {
        VerifierConcrete::Evm { .. } => <EvmVerifier<'a, P, D> as IVerifierAdapter>::DECLARATION,
        #[cfg(any(test, feature = "mocks"))]
        VerifierConcrete::Mock(_) => <MockIVerifierAdapter<'a, D> as IVerifierAdapter>::DECLARATION,
    };
    if !declaration
        .verifier_forms
        .contains(&P::DECLARATION.verifier_group_arithmetic)
    {
        return Err(CreateVerifierErrorReturn::UnsupportedVerifierForm {
            form: P::DECLARATION.verifier_group_arithmetic,
        });
    }
    if !declaration
        .statement_versions
        .contains(&params.statement_version)
    {
        return Err(CreateVerifierErrorReturn::UnsupportedStatementVersion {
            version: params.statement_version,
        });
    }
    match params.concrete {
        VerifierConcrete::Evm { settings } => {
            let adapter = EvmVerifier::try_new(EvmVerifierConstructorParams {
                pairing: deps.pairing,
                delivery_proof: deps.delivery_proof,
                settings: *settings,
            })
            .map_err(CreateVerifierErrorReturn::Evm)?;
            let output = deps
                .consumer
                .consume_verifier(ConsumeVerifierParams, ConsumeVerifierPayload { adapter });
            Ok(CreateVerifierSuccessReturn { output })
        }
        #[cfg(any(test, feature = "mocks"))]
        VerifierConcrete::Mock(failure_mode) => {
            let Ok(adapter) =
                MockIVerifierAdapter::try_new(MockIVerifierAdapterConstructorParams {
                    failure_mode,
                    delivery_proof: deps.delivery_proof,
                });
            let output = deps
                .consumer
                .consume_verifier(ConsumeVerifierParams, ConsumeVerifierPayload { adapter });
            Ok(CreateVerifierSuccessReturn { output })
        }
    }
}
