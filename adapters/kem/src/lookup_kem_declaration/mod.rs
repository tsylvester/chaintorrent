mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::bb1_depth_one::provides::BB1_DEPTH_ONE_DECLARATION;
use crate::factory::provides::KemConcrete;
#[cfg(any(test, feature = "mocks"))]
use crate::factory::provides::MOCK_I_CREDENTIAL_KEM_ADAPTER_DECLARATION;
use interface::{
    LookupKemDeclarationDeps, LookupKemDeclarationParams, LookupKemDeclarationPayload,
    LookupKemDeclarationReturn, LookupKemDeclarationSuccessReturn,
};

pub fn lookup_kem_declaration(
    _deps: &LookupKemDeclarationDeps,
    params: LookupKemDeclarationParams,
    _payload: LookupKemDeclarationPayload,
) -> LookupKemDeclarationReturn {
    let declaration = match params.concrete {
        KemConcrete::Bb1DepthOne => BB1_DEPTH_ONE_DECLARATION,
        #[cfg(any(test, feature = "mocks"))]
        KemConcrete::Mock => MOCK_I_CREDENTIAL_KEM_ADAPTER_DECLARATION,
    };
    Ok(LookupKemDeclarationSuccessReturn { declaration })
}
