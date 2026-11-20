mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use super::evm::provides::EVM_VERIFIER_DECLARATION;
#[cfg(any(test, feature = "mocks"))]
use super::factory::provides::MOCK_I_VERIFIER_ADAPTER_DECLARATION;
use super::factory::provides::VerifierConcreteKind;
use interface::{
    LookupVerifierDeclarationDeps, LookupVerifierDeclarationFn, LookupVerifierDeclarationParams,
    LookupVerifierDeclarationPayload, LookupVerifierDeclarationReturn,
    LookupVerifierDeclarationSuccessReturn,
};

pub fn lookup_verifier_declaration(
    _deps: &LookupVerifierDeclarationDeps,
    params: LookupVerifierDeclarationParams,
    _payload: LookupVerifierDeclarationPayload,
) -> LookupVerifierDeclarationReturn {
    let _: LookupVerifierDeclarationFn = lookup_verifier_declaration;
    match params.identifier {
        VerifierConcreteKind::Evm => Ok(LookupVerifierDeclarationSuccessReturn {
            declaration: EVM_VERIFIER_DECLARATION,
        }),
        #[cfg(any(test, feature = "mocks"))]
        VerifierConcreteKind::Mock => Ok(LookupVerifierDeclarationSuccessReturn {
            declaration: MOCK_I_VERIFIER_ADAPTER_DECLARATION,
        }),
    }
}
