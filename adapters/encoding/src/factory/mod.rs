#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::abi::provides::{AbiEncoding, AbiEncodingConstructorParams};
use crate::lookup_encoding_declaration::provides::{
    LookupEncodingDeclarationDeps, LookupEncodingDeclarationParams,
    LookupEncodingDeclarationPayload,
};
use interface::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingDeps, CreateEncodingParams,
    CreateEncodingPayload, CreateEncodingReturn, CreateEncodingSuccessReturn, EncodingConcrete,
    IEncodingConsumer,
};
#[cfg(any(test, feature = "mocks"))]
use mock::MockIEncoderAdapter;

pub fn create_encoding<C: IEncodingConsumer>(
    deps: &CreateEncodingDeps<C>,
    params: CreateEncodingParams,
    _payload: CreateEncodingPayload,
) -> CreateEncodingReturn<C::Output> {
    let Ok(looked_up) = (deps.lookup)(
        &LookupEncodingDeclarationDeps,
        LookupEncodingDeclarationParams {
            concrete: params.selection,
        },
        LookupEncodingDeclarationPayload,
    );
    let declaration = looked_up.declaration;
    match params.selection {
        EncodingConcrete::Abi => {
            let Ok(adapter) = AbiEncoding::try_new(AbiEncodingConstructorParams);
            Ok(CreateEncodingSuccessReturn {
                output: deps.consumer.consume_encoding(
                    ConsumeEncodingParams,
                    ConsumeEncodingPayload {
                        adapter,
                        declaration,
                    },
                ),
            })
        }
        #[cfg(any(test, feature = "mocks"))]
        EncodingConcrete::Mock => Ok(CreateEncodingSuccessReturn {
            output: deps.consumer.consume_encoding(
                ConsumeEncodingParams,
                ConsumeEncodingPayload {
                    adapter: MockIEncoderAdapter,
                    declaration,
                },
            ),
        }),
    }
}
