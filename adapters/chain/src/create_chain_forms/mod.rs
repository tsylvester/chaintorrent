#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::evm_forms::provides::{EvmForms, EvmFormsConstructorParams};
#[cfg(any(test, feature = "mocks"))]
use crate::factory::provides::{
    IChainForms, MockIChainForms, MockIChainFormsOverrides, build_mock_i_chain_forms,
};
use encoding::IEncoderAdapter;
use interface::{
    ChainFormsConcrete, ConsumeChainFormsParams, ConsumeChainFormsPayload, CreateChainFormsDeps,
    CreateChainFormsErrorReturn, CreateChainFormsParams, CreateChainFormsPayload,
    CreateChainFormsReturn, CreateChainFormsSuccessReturn, IChainFormsConsumer,
};

pub fn create_chain_forms<C: IChainFormsConsumer, E: IEncoderAdapter>(
    deps: &CreateChainFormsDeps<'_, C, E>,
    params: CreateChainFormsParams,
    _payload: CreateChainFormsPayload,
) -> CreateChainFormsReturn<C::Output> {
    match params.concrete {
        ChainFormsConcrete::Evm => {
            if EvmForms::DECLARATION.identifier != params.identifier {
                return Err(CreateChainFormsErrorReturn::UnsupportedChainFormsIdentifier);
            }
            if E::DECLARATION.identifier != EvmForms::DECLARATION.encoding_identifier {
                return Err(CreateChainFormsErrorReturn::UnsupportedEncodingIdentifier);
            }
            let Ok(forms) = EvmForms::try_new(EvmFormsConstructorParams {
                encoder: deps.encoder,
            });
            let output = deps
                .consumer
                .consume_chain_forms(ConsumeChainFormsParams, ConsumeChainFormsPayload { forms });
            Ok(CreateChainFormsSuccessReturn { output })
        }
        #[cfg(any(test, feature = "mocks"))]
        ChainFormsConcrete::Mock => {
            if MockIChainForms::DECLARATION.identifier != params.identifier {
                return Err(CreateChainFormsErrorReturn::UnsupportedChainFormsIdentifier);
            }
            if E::DECLARATION.identifier != MockIChainForms::DECLARATION.encoding_identifier {
                return Err(CreateChainFormsErrorReturn::UnsupportedEncodingIdentifier);
            }
            let forms = build_mock_i_chain_forms(MockIChainFormsOverrides);
            let output = deps
                .consumer
                .consume_chain_forms(ConsumeChainFormsParams, ConsumeChainFormsPayload { forms });
            Ok(CreateChainFormsSuccessReturn { output })
        }
    }
}
