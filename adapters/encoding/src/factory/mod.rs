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
    CanonicalFieldKind, CanonicalFieldValue, ConsumeEncodingParams, ConsumeEncodingPayload,
    CreateEncodingDeps, CreateEncodingErrorReturn, CreateEncodingParams, CreateEncodingPayload,
    CreateEncodingReturn, CreateEncodingSuccessReturn, EncodingConcrete, EncodingTypeName,
    EncodingTypeNameConstructorParams, EncodingTypeNameErrorReturn, EncodingTypeNameTryNewReturn,
    IEncodingConsumer,
};
#[cfg(any(test, feature = "mocks"))]
use mock::MockIEncoderAdapter;

impl EncodingTypeName {
    pub fn try_new(params: EncodingTypeNameConstructorParams) -> EncodingTypeNameTryNewReturn {
        if params.name.is_empty() {
            return Err(EncodingTypeNameErrorReturn::Empty);
        }
        if !params
            .name
            .chars()
            .all(|character| character.is_ascii_alphanumeric())
        {
            return Err(EncodingTypeNameErrorReturn::NotAlphanumeric);
        }
        Ok(EncodingTypeName { name: params.name })
    }

    pub fn as_str(&self) -> &str {
        self.name.as_str()
    }
}

impl CanonicalFieldValue {
    pub fn kind(&self) -> CanonicalFieldKind {
        match self {
            CanonicalFieldValue::FixedBytes32(_) => CanonicalFieldKind::FixedBytes32,
            CanonicalFieldValue::Unsigned16(_) => CanonicalFieldKind::Unsigned16,
            CanonicalFieldValue::Unsigned32(_) => CanonicalFieldKind::Unsigned32,
            CanonicalFieldValue::Unsigned64(_) => CanonicalFieldKind::Unsigned64,
            CanonicalFieldValue::Text(_) => CanonicalFieldKind::Text,
            CanonicalFieldValue::Bytes(_) => CanonicalFieldKind::Bytes,
            CanonicalFieldValue::FixedBytes20(_) => CanonicalFieldKind::FixedBytes20,
            CanonicalFieldValue::Unsigned256(_) => CanonicalFieldKind::Unsigned256,
        }
    }
}

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
            Ok(CreateEncodingSuccessReturn {
                output: deps
                    .consumer
                    .consume_encoding(ConsumeEncodingParams, ConsumeEncodingPayload { adapter }),
            })
        }
        #[cfg(any(test, feature = "mocks"))]
        EncodingConcrete::Mock => {
            if MockIEncoderAdapter::DECLARATION.identifier != params.identifier {
                return Err(CreateEncodingErrorReturn::UnsupportedEncodingIdentifier);
            }
            Ok(CreateEncodingSuccessReturn {
                output: deps.consumer.consume_encoding(
                    ConsumeEncodingParams,
                    ConsumeEncodingPayload {
                        adapter: MockIEncoderAdapter,
                    },
                ),
            })
        }
    }
}
