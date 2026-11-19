mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use super::factory::provides::GenerateConcreteKind;
#[cfg(any(test, feature = "mocks"))]
use super::factory::provides::MOCK_I_GENERATE_ADAPTER_DECLARATION;
use super::solidity::provides::SOLIDITY_GENERATE_DECLARATION;
use interface::{
    LookupGenerateDeclarationDeps, LookupGenerateDeclarationFn, LookupGenerateDeclarationParams,
    LookupGenerateDeclarationPayload, LookupGenerateDeclarationReturn,
    LookupGenerateDeclarationSuccessReturn,
};

pub fn lookup_generate_declaration(
    _deps: &LookupGenerateDeclarationDeps,
    params: LookupGenerateDeclarationParams,
    _payload: LookupGenerateDeclarationPayload,
) -> LookupGenerateDeclarationReturn {
    let _: LookupGenerateDeclarationFn = lookup_generate_declaration;
    match params.identifier {
        GenerateConcreteKind::Solidity => Ok(LookupGenerateDeclarationSuccessReturn {
            declaration: SOLIDITY_GENERATE_DECLARATION,
        }),
        #[cfg(any(test, feature = "mocks"))]
        GenerateConcreteKind::Mock => Ok(LookupGenerateDeclarationSuccessReturn {
            declaration: MOCK_I_GENERATE_ADAPTER_DECLARATION,
        }),
    }
}
