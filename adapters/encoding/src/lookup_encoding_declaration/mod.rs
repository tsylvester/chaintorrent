mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::abi::provides::AbiEncoding;
use crate::factory::provides::EncodingConcrete;
#[cfg(any(test, feature = "mocks"))]
use crate::factory::provides::MOCK_I_ENCODER_ADAPTER_DECLARATION;
use interface::{
    LookupEncodingDeclarationDeps, LookupEncodingDeclarationParams,
    LookupEncodingDeclarationPayload, LookupEncodingDeclarationReturn,
    LookupEncodingDeclarationSuccessReturn,
};

pub fn lookup_encoding_declaration(
    _deps: &LookupEncodingDeclarationDeps,
    params: LookupEncodingDeclarationParams,
    _payload: LookupEncodingDeclarationPayload,
) -> LookupEncodingDeclarationReturn {
    match params.concrete {
        EncodingConcrete::Abi => Ok(LookupEncodingDeclarationSuccessReturn {
            declaration: AbiEncoding::DECLARATION,
        }),
        #[cfg(any(test, feature = "mocks"))]
        EncodingConcrete::Mock => Ok(LookupEncodingDeclarationSuccessReturn {
            declaration: MOCK_I_ENCODER_ADAPTER_DECLARATION,
        }),
    }
}
