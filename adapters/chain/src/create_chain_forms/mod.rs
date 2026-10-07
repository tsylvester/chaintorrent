mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::evm_forms::provides::{EvmForms, EvmFormsConstructorParams};
use interface::{
    ChainFormsConcrete, ConsumeChainFormsParams, ConsumeChainFormsPayload, CreateChainFormsDeps,
    CreateChainFormsErrorReturn, CreateChainFormsParams, CreateChainFormsPayload,
    CreateChainFormsReturn, CreateChainFormsSuccessReturn, IChainFormsConsumer,
};

pub fn create_chain_forms<C: IChainFormsConsumer>(
    deps: &CreateChainFormsDeps<C>,
    params: CreateChainFormsParams,
    _payload: CreateChainFormsPayload,
) -> CreateChainFormsReturn<C::Output> {
    match params.concrete {
        ChainFormsConcrete::Evm => {
            if EvmForms::DECLARATION.identifier != params.identifier {
                return Err(CreateChainFormsErrorReturn::UnsupportedChainFormsIdentifier);
            }
            let Ok(forms) = EvmForms::try_new(EvmFormsConstructorParams);
            let output = deps
                .consumer
                .consume_chain_forms(ConsumeChainFormsParams, ConsumeChainFormsPayload { forms });
            Ok(CreateChainFormsSuccessReturn { output })
        }
    }
}
