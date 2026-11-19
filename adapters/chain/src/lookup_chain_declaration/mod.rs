mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::base::provides::BASE_CHAIN_DECLARATION;
use crate::factory::provides::ChainMemberIdentifier;
#[cfg(any(test, feature = "mocks"))]
use crate::factory::provides::MOCK_I_CHAIN_ADAPTER_DECLARATION;
use interface::{
    LookupChainDeclarationDeps, LookupChainDeclarationParams, LookupChainDeclarationPayload,
    LookupChainDeclarationReturn, LookupChainDeclarationSuccessReturn,
};

pub fn lookup_chain_declaration(
    _deps: &LookupChainDeclarationDeps,
    params: LookupChainDeclarationParams,
    _payload: LookupChainDeclarationPayload,
) -> LookupChainDeclarationReturn {
    match params.identifier {
        ChainMemberIdentifier::Base => Ok(LookupChainDeclarationSuccessReturn {
            declaration: BASE_CHAIN_DECLARATION,
        }),
        #[cfg(any(test, feature = "mocks"))]
        ChainMemberIdentifier::Mock => Ok(LookupChainDeclarationSuccessReturn {
            declaration: MOCK_I_CHAIN_ADAPTER_DECLARATION,
        }),
    }
}
