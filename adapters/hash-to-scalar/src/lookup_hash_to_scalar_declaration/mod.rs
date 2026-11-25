mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

#[cfg(any(test, feature = "mocks"))]
use crate::factory::provides::MOCK_I_HASH_TO_SCALAR_ADAPTER_DECLARATION;
use crate::factory::provides::{HashToScalarConcrete, HashToScalarDeclaration};
use crate::keccak256::provides::Keccak256HashToScalar;
use interface::{
    LookupHashToScalarDeclarationDeps, LookupHashToScalarDeclarationParams,
    LookupHashToScalarDeclarationPayload, LookupHashToScalarDeclarationReturn,
    LookupHashToScalarDeclarationSuccessReturn,
};

pub fn lookup_hash_to_scalar_declaration(
    _deps: &LookupHashToScalarDeclarationDeps,
    params: LookupHashToScalarDeclarationParams,
    _payload: LookupHashToScalarDeclarationPayload,
) -> LookupHashToScalarDeclarationReturn {
    match params.concrete {
        HashToScalarConcrete::Keccak256 => Ok(LookupHashToScalarDeclarationSuccessReturn {
            declaration: Keccak256HashToScalar::DECLARATION,
        }),
        #[cfg(any(test, feature = "mocks"))]
        HashToScalarConcrete::Mock(mode) => Ok(LookupHashToScalarDeclarationSuccessReturn {
            declaration: HashToScalarDeclaration {
                concrete: HashToScalarConcrete::Mock(mode),
                identifier: MOCK_I_HASH_TO_SCALAR_ADAPTER_DECLARATION.identifier,
                adapter_version: MOCK_I_HASH_TO_SCALAR_ADAPTER_DECLARATION.adapter_version,
                interface_version: MOCK_I_HASH_TO_SCALAR_ADAPTER_DECLARATION.interface_version,
            },
        }),
    }
}
