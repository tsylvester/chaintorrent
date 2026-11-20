#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use chain::{IChainAdapter, IChainForms, IEntitlementStateAdapter};
use pairing::IPairingAdapter;
use proof::IDeliveryProofAdapter;

use super::admit_verifier_composition::provides::{
    ADMIT_VERIFIER_COMPOSITION_DEPS, AdmitVerifierCompositionParams,
    AdmitVerifierCompositionPayload,
};
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
    C: IChainAdapter,
    D: IDeliveryProofAdapter<Pairing = P, Forms = <C as IEntitlementStateAdapter>::Forms>,
    K: IVerifierConsumer<D>,
>(
    deps: &CreateVerifierDeps<'a, P, C, D, K>,
    params: CreateVerifierParams,
    _payload: CreateVerifierPayload,
) -> CreateVerifierReturn<K::Output> {
    let declaration = match &params.concrete {
        VerifierConcrete::Evm { .. } => <EvmVerifier<'a, P, C, D> as IVerifierAdapter>::DECLARATION,
        #[cfg(any(test, feature = "mocks"))]
        VerifierConcrete::Mock(_) => <MockIVerifierAdapter<'a, D> as IVerifierAdapter>::DECLARATION,
    };
    if let Err(error) = (deps.admit_verifier_composition)(
        &ADMIT_VERIFIER_COMPOSITION_DEPS,
        AdmitVerifierCompositionParams {
            identifier: declaration.identifier,
        },
        AdmitVerifierCompositionPayload {
            chain_forms: &<<C as IEntitlementStateAdapter>::Forms as IChainForms>::DECLARATION,
            chain: &C::DECLARATION,
            encoder: deps.encoder,
            pairing: &P::DECLARATION,
            generate: deps.generate,
        },
    ) {
        return Err(CreateVerifierErrorReturn::Composition(error));
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
                chain: deps.chain,
                delivery_proof: deps.delivery_proof,
                verifier_address: params.verifier_address,
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
