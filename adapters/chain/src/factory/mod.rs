#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::admit_chain_composition::provides::{
    ADMIT_CHAIN_COMPOSITION_DEPS, AdmitChainCompositionParams, AdmitChainCompositionPayload,
};
use crate::base::provides::{BaseChain, BaseChainConstructorParams};
use interface::{
    ChainMemberIdentifier, ChainSelection, ConsumeChainParams, ConsumeChainPayload,
    CreateChainDeps, CreateChainErrorReturn, CreateChainParams, CreateChainPayload,
    CreateChainReturn, CreateChainSuccessReturn, IChainConsumer, IChainForms,
};
#[cfg(any(test, feature = "mocks"))]
use mock::{MockIChainAdapterOverrides, build_mock_i_chain_adapter};

pub fn create_chain<F: IChainForms, C: IChainConsumer>(
    deps: &CreateChainDeps<C>,
    params: CreateChainParams,
    _payload: CreateChainPayload,
) -> CreateChainReturn<C::Output> {
    let identifier = match &params.selection {
        ChainSelection::Base { .. } => ChainMemberIdentifier::Base,
        #[cfg(any(test, feature = "mocks"))]
        ChainSelection::Mock { .. } => ChainMemberIdentifier::Mock,
    };
    if let Err(error) = (deps.admit_chain_composition)(
        &ADMIT_CHAIN_COMPOSITION_DEPS,
        AdmitChainCompositionParams { identifier },
        AdmitChainCompositionPayload {
            forms: &F::DECLARATION,
        },
    ) {
        return Err(CreateChainErrorReturn::Composition(error));
    }
    match params.selection {
        ChainSelection::Base { settings } => {
            let adapter = match BaseChain::<F>::try_new(BaseChainConstructorParams {
                settings: &settings,
            }) {
                Err(error) => return Err(CreateChainErrorReturn::Base(error)),
                Ok(adapter) => adapter,
            };
            Ok(CreateChainSuccessReturn {
                output: deps
                    .consumer
                    .consume_chain(ConsumeChainParams, ConsumeChainPayload { adapter }),
            })
        }
        #[cfg(any(test, feature = "mocks"))]
        ChainSelection::Mock { failure_mode } => {
            let adapter = build_mock_i_chain_adapter::<F>(MockIChainAdapterOverrides {
                failure_mode: Some(failure_mode),
            });
            Ok(CreateChainSuccessReturn {
                output: deps
                    .consumer
                    .consume_chain(ConsumeChainParams, ConsumeChainPayload { adapter }),
            })
        }
    }
}
