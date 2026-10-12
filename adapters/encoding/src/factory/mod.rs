#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::abi::provides::{AbiEncoding, AbiEncodingConstructorParams};
#[cfg(any(test, feature = "mocks"))]
use interface::IEncoderAdapter;
use interface::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingDeps, CreateEncodingErrorReturn,
    CreateEncodingParams, CreateEncodingPayload, CreateEncodingReturn, CreateEncodingSuccessReturn,
    EncodingConcrete, IEncodingConsumer,
};
#[cfg(any(test, feature = "mocks"))]
use mock::MockIEncoderAdapter;

pub fn create_encoding<C: IEncodingConsumer>(
    deps: &CreateEncodingDeps<C>,
    params: CreateEncodingParams,
    _payload: CreateEncodingPayload,
) -> CreateEncodingReturn<C::Output> {
    match params.concrete {
        EncodingConcrete::Abi => {
            if AbiEncoding::DECLARATION.identifier != params.identifier {
                return Err(CreateEncodingErrorReturn::UnsupportedEncodingIdentifier);
            }
            let Ok(adapter) = AbiEncoding::try_new(AbiEncodingConstructorParams);
            let output = deps
                .consumer
                .consume_encoding(ConsumeEncodingParams, ConsumeEncodingPayload { adapter });
            Ok(CreateEncodingSuccessReturn { output })
        }
        #[cfg(any(test, feature = "mocks"))]
        EncodingConcrete::Mock => {
            if MockIEncoderAdapter::DECLARATION.identifier != params.identifier {
                return Err(CreateEncodingErrorReturn::UnsupportedEncodingIdentifier);
            }
            let output = deps.consumer.consume_encoding(
                ConsumeEncodingParams,
                ConsumeEncodingPayload {
                    adapter: MockIEncoderAdapter,
                },
            );
            Ok(CreateEncodingSuccessReturn { output })
        }
    }
}
